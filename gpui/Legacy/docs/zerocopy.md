Below is an **AI-agent-friendly playbook** to implement *real* zero-copy in Rust (borrow views over an input buffer, copy only when forced). It’s written as steps + checklists an agent can execute.

---

## Zero-copy Rust playbook (agent checklist)

### 0) Define the “no-copy boundary”

**Goal:** after bytes enter the system, everything downstream should be *views* until you explicitly cross an “ownership boundary”.

**Agent tasks**

* Locate all allocations/copies on the hot path:

  * `to_vec()`, `to_string()`, `String::from_utf8`, `serde_json::from_slice` (often alloc-heavy), `clone()` on buffers.
* Mark the “ownership boundary” where copying is allowed (e.g., cache storage, DB write, mutation).

**Rule:** “Borrow by default; copy only at boundaries.”

---

## 1) Own the buffer once, then only slice it

### Best buffer choices

* **`bytes::Bytes` / `BytesMut`** (excellent for networking + cheap slicing via refcount)
* **`Arc<[u8]>`** (simple shared ownership)
* **`Vec<u8>`** (fine if you never reallocate after creating borrows)

**Critical rule:** once you hand out `&[u8]`/`&str` borrows, **do not mutate/resize** the underlying `Vec<u8>` (realloc can invalidate references).

**Agent tasks**

* Convert incoming data into a stable owner type early:

  * network: receive into `BytesMut`, freeze into `Bytes`
  * file: mmap (if used), or read into `Vec<u8>` once
* Ensure the owner outlives all parsed views.

---

## 2) Parse into *borrowed view types* (the main zero-copy pattern)

### Pattern

Create structs that **borrow from the buffer**:

* Use `&'a [u8]` for binary slices
* Use `&'a str` only when validated UTF-8
* Keep indices/ranges if you need to delay slicing

**Agent tasks**

* Define `struct Parsed<'a> { ... }` with borrowed fields
* Implement `fn parse<'a>(buf: &'a [u8]) -> Result<Parsed<'a>, Error>`
* Validate bounds early (length checks) to avoid panics and UB.

**Optimization notes**

* Avoid “extract then allocate”: don’t build `String`s for tokens; keep `&str`.
* For substring search, prefer **`memchr`**-style scanning to reduce overhead (if relevant).

---

## 3) Use `Cow` for “borrow fast-path, own when necessary”

### When to use

If you *usually* can borrow, but sometimes need to:

* normalize (lowercase/trim/escape)
* store beyond input lifetime
* mutate

Use: `Cow<'a, str>` / `Cow<'a, [u8]>`

**Agent tasks**

* Replace “always allocate” outputs with `Cow`
* Only allocate on the slow-path (`to_mut()` / transformation step)

---

## 4) Zero-copy binary deserialization (fast, but strict)

If the data format is fixed-layout and you want `&T` from `&[u8]`, use:

* `zerocopy` (safer read-from-bytes APIs)
* `bytemuck` (POD casting; strict invariants)

### Mandatory constraints checklist

* `#[repr(C)]` or `#[repr(transparent)]` as required
* Alignment must be correct (don’t assume `&[u8]` is aligned for `T`)
* Handle endianness explicitly (wire formats ≠ host)
* Avoid pointers/references inside the casted struct unless using approved patterns
* Padding must be consistent (often add explicit padding fields)

**Agent tasks**

* If using casting: implement a safe “read” layer that:

  * checks length
  * checks alignment / uses unaligned reads when appropriate
  * converts endianness (e.g., wrap fields or post-process)

---

## 5) Lifetime strategy: keep it simple and mechanical

### The safe default

* Parsing functions return `Parsed<'a>` borrowing from input `&'a [u8]`
* Callers keep the owned buffer alive while using `Parsed`

**Agent tasks**

* Make lifetimes explicit in public APIs (don’t fight elision when it gets confusing)
* Never return borrows to temporaries:

  * don’t borrow from a local `Vec` that gets dropped
  * don’t create a `String` then return `&str` to it

---

## 6) Avoid self-referential structs unless forced

If you need a struct that **owns bytes and also stores borrows into itself**, that’s self-referential and hard in safe Rust.

Preferred options:

1. **Return views that borrow from an external owner** (simplest)
2. Use `yoke` to tie borrowed data to an owned container safely
3. Use `ouroboros` only if you truly need self-references (last resort)

**Agent tasks**

* Refactor to “owner outside + views returned” before reaching for advanced crates
* If forced, apply `yoke`/`ouroboros` with minimal surface area

---

## 7) Operational optimizations agents often miss

### Minimize copies across layers

* Keep bytes as bytes; postpone UTF-8 decode until needed (`std::str::from_utf8`)
* Don’t turn `&[u8]` into `Vec<u8>` unless you must own/mutate
* Prefer “slice + length” over building new buffers

### Ensure you’re actually faster

**Agent tasks**

* Add benches for the hot path (criterion or built-in benches)
* Track allocations (e.g., by logging, profiling, or allocator stats)
* Fuzz inputs (zero-copy parsing is often boundary-check sensitive)

---

## Decision tree (agent quick pick)

* **Text-ish parsing (log lines, JSON-ish tokens, protocol strings)** → borrowed `&str`/`&[u8]` views + `Cow` for occasional ownership
* **Network frames / shared slicing** → `Bytes` (cheap clone + slice)
* **Fixed-layout binary headers** → `zerocopy` / `bytemuck` (with alignment + endian discipline)
* **Need views tied to owned storage** → owner + views, or `yoke`
* **Self-referential unavoidable** → `ouroboros` (minimize usage)
