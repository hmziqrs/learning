# GitHub Actions Workflows

> **⚠️ ARCHIVED:** The E2E testing workflow has been archived and is not currently running in CI. The configuration is available at `e2e-tests.yml.archive` for future reference and will be fixed later.

## E2E Testing Workflow (ARCHIVED)

The `e2e-tests.yml.archive` workflow previously ran comprehensive end-to-end tests for the Tauri app on **Linux** and **Windows** (since macOS is not supported by tauri-driver).

### When It Runs

> **⚠️ Currently DISABLED** - The workflow has been archived and will not run automatically.

Previously ran on:
- **Every push** to `master` or `claude/**` branches (only if `tauri-app/**` files change)
- **Pull requests** to `master` (only if `tauri-app/**` files change)
- **Manual trigger** via GitHub Actions UI (workflow_dispatch)

**Note:** Tests only ran when changes were made in the `tauri-app/` directory. Changes to other parts of the repo wouldn't trigger E2E tests.

### What It Does

1. **Sets up environment** (Linux dependencies, Rust, Bun)
2. **Installs tauri-driver** (WebDriver for Tauri apps)
3. **Builds the Tauri app** in debug mode
4. **Runs E2E tests** using WebdriverIO
5. **Saves logs and screenshots** as artifacts

### Platforms

| Platform | Status | Notes |
|----------|--------|-------|
| **Linux** | ✅ Enabled | Uses Xvfb for headless testing |
| **Windows** | ✅ Enabled | Native testing via tauri-driver |
| **macOS** | ❌ Disabled | tauri-driver doesn't support macOS |

### Viewing Test Results

#### 1. Check Workflow Status

Go to the **Actions** tab in your GitHub repository:
```
https://github.com/YOUR_USERNAME/learning/actions
```

You'll see:
- ✅ Green checkmark = All tests passed
- ❌ Red X = Tests failed
- 🟡 Yellow dot = Tests running

#### 2. View Test Logs

Click on a workflow run → Click on a job (e.g., "E2E Tests (ubuntu-latest)") → Expand steps to see detailed logs.

The **"Run E2E tests"** step shows:
- Test execution output
- Which tests passed/failed
- Error messages and stack traces

#### 3. Download Artifacts

At the bottom of each workflow run, you'll find **Artifacts**:

**Artifacts saved:**
- `e2e-test-logs-ubuntu-latest` - Complete test output log (Linux)
- `e2e-test-logs-windows-latest` - Complete test output log (Windows)
- `e2e-screenshots-*-failure` - Screenshots from failed tests (if any)

**To download:**
1. Go to workflow run page
2. Scroll to bottom
3. Click artifact name to download ZIP
4. Extract and view logs/screenshots

#### 4. Artifact Contents

**Test logs (`e2e-test-output.log`):**
```
🚀 Starting tauri-driver...
📦 Testing binary: /path/to/binary
🧪 Starting Tauri E2E tests...

▶️  Running: should launch the application successfully
✅ Test passed: should launch the application successfully (1234ms)

▶️  Running: should create a folder successfully
✅ Test passed: should create a folder successfully (567ms)

Spec Files:      10 passed, 0 failed, 10 total
```

**Screenshots (on failure):**
```
e2e/screenshots/
  failed-should-create-folder-2025-11-17T08-30-00.png
  failed-should-read-file-2025-11-17T08-30-15.png
```

### Triggering Manual Runs

You can manually trigger the workflow:

1. Go to **Actions** tab
2. Click **E2E Tests** workflow
3. Click **Run workflow** button
4. Select branch
5. Click green **Run workflow** button

### Test Configuration

The workflow uses these test scripts (from `tauri-app/package.json`):

```json
{
  "test:e2e": "wdio run ./wdio.conf.ts",
  "test:e2e:smoke": "wdio run ./wdio.conf.ts --spec ./e2e/specs/smoke.spec.ts",
  "test:e2e:filesystem": "wdio run ./wdio.conf.ts --spec ./e2e/specs/filesystem.spec.ts"
}
```

Currently runs **all tests** (`test:e2e`). To run specific suites, modify the workflow file.

### Troubleshooting

#### Tests Fail on Linux

**Issue:** `webkit2gtk-driver` or Xvfb errors

**Fix:** The workflow already installs required dependencies. If tests still fail, check:
- Build step succeeded?
- Binary exists at expected path?
- Test logs for specific error messages

#### Tests Fail on Windows

**Issue:** WebDriver connection errors or tauri-driver errors

**Fix:** Windows doesn't need a separate browser driver - tauri-driver handles everything. Check:
- tauri-driver installed successfully?
- Binary built correctly?
- Test logs for specific error messages

#### Build Takes Too Long

The workflow has a **30-minute timeout**. If builds exceed this:
- Check Rust cache is working (Swatinem/rust-cache)
- Consider splitting into separate build/test jobs
- Use release builds (`tauri:build`) for faster execution

#### Can't Find Artifacts

Artifacts are retained for **30 days**. After that, they're automatically deleted. Download them promptly if you need them.

### Local Testing vs CI

**Differences:**

| Aspect | Local (macOS) | CI (Linux/Windows) |
|--------|---------------|-------------------|
| tauri-driver | ❌ Not supported | ✅ Works |
| Test execution | Can't run E2E | ✅ Runs fully |
| Artifacts | Manual save | ✅ Auto-saved |
| Parallel tests | N/A | ✅ Linux + Windows |

**Recommendation:** Always check CI results even if you can't test locally on macOS.

### Customizing the Workflow

#### Run on Different Branches

Edit `on.push.branches`:
```yaml
on:
  push:
    branches:
      - master
      - staging
      - 'feature/**'
```

#### Change Test Command

Edit the test execution step:
```yaml
- name: Run E2E tests
  run: bun run test:e2e:smoke  # Run smoke tests only
```

#### Add More Platforms

Currently runs on:
- `ubuntu-latest` (Linux)
- `windows-latest` (Windows)

To add specific versions:
```yaml
matrix:
  os: [ubuntu-22.04, windows-2022, windows-2019]
```

#### Adjust Timeouts

Modify at job level:
```yaml
jobs:
  e2e-tests:
    timeout-minutes: 45  # Increase to 45 minutes
```

### Performance Optimization

The workflow uses:
- ✅ **Rust caching** (Swatinem/rust-cache) - Caches compiled dependencies
- ✅ **Frozen lockfile** - Ensures reproducible builds
- ✅ **Parallel execution** - Linux and Windows run simultaneously
- ✅ **Debug builds** - Faster than release builds for testing

**Build times:**
- First run: ~10-15 minutes (no cache)
- Subsequent runs: ~3-5 minutes (with cache)

### Security Notes

The workflow can access these secrets (if set):
- `TAURI_SIGNING_PRIVATE_KEY` - For code signing
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` - Signing key password

These are optional for debug builds but recommended for release builds.

### Status Badge

Add this to your README to show test status:

```markdown
![E2E Tests](https://github.com/YOUR_USERNAME/learning/actions/workflows/e2e-tests.yml/badge.svg)
```

Replace `YOUR_USERNAME` with your GitHub username.

## Need Help?

- **Test failures:** Check artifacts for detailed logs
- **Build issues:** Review build step output
- **Configuration questions:** See tauri-app/e2e/README.md
- **macOS testing:** See tauri-app/e2e/MACOS-TESTING.md

---

**Happy Testing!** 🧪✨
