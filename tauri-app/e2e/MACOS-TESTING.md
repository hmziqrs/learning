# E2E Testing on macOS - Known Limitations

## ⚠️ Important: tauri-driver Doesn't Support macOS

**`tauri-driver` currently does not support macOS.** When you try to run E2E tests on macOS, you'll see:

```
tauri-driver is not supported on this platform
```

This is a known limitation of tauri-driver. Native app E2E testing with tauri-driver works on:
- ✅ **Linux** (full support)
- ✅ **Windows** (full support)
- ❌ **macOS** (not supported)

## 🔧 Workarounds for macOS Developers

### Option 1: Test on Linux/Windows (Recommended)

Run your E2E tests on Linux or Windows machines:

**Using GitHub Actions (CI/CD):**
```yaml
name: E2E Tests
on: [push, pull_request]
jobs:
  test-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run tauri:build:debug
      - run: bun run test:e2e

  test-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: oven-sh/setup-bun@v1
      - run: bun install
      - run: bun run tauri:build:debug
      - run: bun run test:e2e
```

**Using Docker (Linux):**
```bash
# Run tests in a Linux container
docker run -it --rm \
  -v $(pwd):/app \
  -w /app \
  rust:latest \
  bash -c "apt-get update && apt-get install -y nodejs npm && npm install -g bun && bun install && bun run tauri:build:debug && bun run test:e2e"
```

### Option 2: Use Component/Integration Tests

Instead of full E2E tests, use:

1. **Vitest** for component testing
2. **React Testing Library** for UI testing
3. **Tauri's mock utilities** for Rust command testing

These don't require tauri-driver and work perfectly on macOS.

### Option 3: Manual Testing on macOS

Build and test manually:
```bash
# Build the app
bun run tauri:build:debug

# Open and test manually
open src-tauri/target/debug/bundle/macos/tauri-app.app
```

### Option 4: Remote Testing

Use a remote Linux/Windows machine:
- Cloud CI/CD (GitHub Actions, CircleCI, etc.)
- Remote desktop to Windows/Linux machine
- Virtual machine (Parallels, VMware with Linux/Windows)

## 📊 Testing Strategy for Cross-Platform Apps

**Recommended approach:**

1. **Local Development (macOS)**:
   - Component tests (Vitest + Testing Library)
   - Integration tests (Tauri mock utilities)
   - Manual smoke testing

2. **CI/CD (Linux + Windows)**:
   - Full E2E test suite
   - Automated on every PR
   - Tests real native behavior

3. **Before Release**:
   - Manual testing on all platforms
   - E2E tests pass on Linux + Windows
   - Smoke tests on macOS

## 🔮 Future Support

Track macOS support progress:
- [Tauri WebDriver Issue](https://github.com/tauri-apps/tauri/issues/webdriver)
- [tauri-driver Repository](https://github.com/tauri-apps/tauri-driver)

## 🚀 Alternative: Test the Dev Server

If you must test on macOS, you can test against the development server (not the compiled binary):

**Run the dev server:**
```bash
bun run tauri:dev
```

Then use standard WebDriver tools to test the webview content (but this doesn't test native app functionality).

## ❓ FAQ

**Q: Why doesn't tauri-driver support macOS?**
A: macOS has stricter security and automation restrictions. Implementing WebDriver for macOS requires different approaches (like using XCUITest).

**Q: Can I still develop on macOS?**
A: Absolutely! Just run E2E tests in CI/CD on Linux/Windows.

**Q: Will macOS support be added?**
A: It's being considered, but no timeline yet. Follow the Tauri repo for updates.

**Q: What about iOS testing?**
A: iOS testing is also limited. Use XCUITest directly or Appium for iOS-specific testing.

---

**Recommendation:** If you're developing cross-platform Tauri apps on macOS, set up CI/CD with Linux/Windows runners for E2E tests. This ensures your tests run on supported platforms automatically.
