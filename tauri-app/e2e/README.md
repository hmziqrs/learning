# E2E Testing for Tauri App

This directory contains end-to-end (E2E) tests for the Tauri application using **WebdriverIO** and **tauri-driver**.

> **⚠️ macOS Users:** tauri-driver does not support macOS. If you're on macOS, see [MACOS-TESTING.md](./MACOS-TESTING.md) for workarounds (run tests on Linux/Windows via CI/CD, Docker, or VM).

## Overview

The E2E test suite **launches and tests the actual native Tauri application**, simulating real user interactions exactly as a user would interact with the desktop app. Tests run against the **real binary** (not a browser), ensuring genuine end-to-end validation.

### Platform Support
- **Linux** ✅ Full support (via tauri-driver)
- **Windows** ✅ Full support (via tauri-driver)
- **macOS** ❌ **Not supported** - tauri-driver doesn't work on macOS ([see workarounds](./MACOS-TESTING.md))
- **Android** ✅ Supported (via Appium)
- **iOS** ⚠️ Limited support (not currently implemented)

## Project Structure

```
e2e/
├── specs/              # Test specifications
│   ├── smoke.spec.ts           # Smoke tests for basic functionality
│   └── filesystem.spec.ts      # Comprehensive filesystem module tests
├── pageobjects/        # Page Object Model (POM)
│   └── filesystem.page.ts      # Filesystem page interactions
├── helpers/            # Test utilities
│   └── test-helpers.ts         # Common helper functions
├── screenshots/        # Test failure screenshots (auto-generated)
├── .gitignore         # Ignore test artifacts
└── README.md          # This file
```

## Prerequisites

### Desktop Testing

1. **Install tauri-driver** (Rust tool for testing native Tauri apps):
   ```bash
   cargo install tauri-driver
   ```

   This is already installed in this project.

2. **Build the Tauri app** (debug or release):
   ```bash
   bun run tauri:build:debug    # Debug build (faster, recommended for testing)
   bun run tauri:build          # Release build
   ```

3. **Dependencies** are already installed via:
   ```bash
   bun install
   ```

### How It Works

The test setup uses **tauri-driver**, which:
1. **Launches the actual Tauri binary** (the .exe, .app, or Linux executable)
2. **Controls the native app** through WebDriver protocol
3. **Simulates real user interactions** (clicks, typing, navigation)
4. **Takes screenshots** and verifies UI state

This is **not browser testing** - it tests the actual compiled desktop application!

### Android Testing

1. **Android Studio** with Android SDK
2. **Appium** globally installed:
   ```bash
   npm install -g appium
   appium driver install uiautomator2
   ```

3. **Android Emulator** running or physical device connected
4. **Build the Android APK**:
   ```bash
   bun run tauri:android:build
   ```

## Running Tests

### Desktop Tests

Run all E2E tests:
```bash
bun run test:e2e
```

Run smoke tests only (quick verification):
```bash
bun run test:e2e:smoke
```

Run filesystem module tests:
```bash
bun run test:e2e:filesystem
```

Build and test in one command:
```bash
bun run test:build-and-test         # Debug build + tests
bun run test:build-and-test:release # Release build + tests
```

### Android Tests

1. **Start an emulator** or connect a device:
   ```bash
   # List available emulators
   emulator -list-avds

   # Start an emulator
   emulator -avd Pixel_9a
   ```

2. **Run Android tests**:
   ```bash
   bun run test:e2e:android
   ```

## Writing Tests

### Test Structure

Tests follow the Mocha BDD style:

```typescript
describe('Feature Name', () => {
  before(async () => {
    // Setup before all tests in this suite
  })

  beforeEach(async () => {
    // Setup before each test
  })

  it('should do something', async () => {
    // Test implementation
    const element = await $('selector')
    await element.click()
    expect(await element.getText()).toBe('Expected text')
  })

  afterEach(async () => {
    // Cleanup after each test
  })

  after(async () => {
    // Cleanup after all tests
  })
})
```

### Using Page Objects

Page Objects encapsulate page interactions for better maintainability:

```typescript
import FilesystemPage from '../pageobjects/filesystem.page.js'

it('should create a file', async () => {
  await FilesystemPage.navigate()
  await FilesystemPage.writeFileFlow('test.txt', 'content')

  const output = await FilesystemPage.getOutputText()
  expect(output).toContain('✓')
})
```

### Using Helpers

Helper functions provide common operations:

```typescript
import { waitAndClick, waitForText, takeScreenshot } from '../helpers/test-helpers.js'

it('should perform action', async () => {
  await waitAndClick('button*=Submit')
  await waitForText('Success')
  await takeScreenshot('success-state')
})
```

### Best Practices

1. **Use Page Objects** for all UI interactions
2. **Avoid hardcoded waits** - use WebdriverIO's built-in waiting mechanisms
3. **Take screenshots on failure** - automatically handled in afterEach hooks
4. **Use descriptive test names** - "should do X when Y happens"
5. **Keep tests independent** - each test should be able to run in isolation
6. **Clean up after tests** - reset state or remove test data

## Debugging Tests

### Run Tests in Visible Mode

Edit `wdio.conf.ts` and remove the `--headless=new` flag:

```typescript
args: [
  // '--headless=new', // Comment this out
  '--disable-gpu',
  '--no-sandbox',
],
```

### Enable Debug Logging

Set log level to debug in `wdio.conf.ts`:

```typescript
logLevel: 'debug',
```

### Pause Test Execution

Add breakpoints in your tests:

```typescript
it('should debug something', async () => {
  await FilesystemPage.navigate()
  await browser.debug() // Pauses here - use REPL to inspect
  await FilesystemPage.clickWriteFile()
})
```

### View Screenshots

Failed tests automatically save screenshots to `e2e/screenshots/`

### View Logs

Check the WebdriverIO logs and Appium logs (for Android):
```bash
# Appium logs are saved to appium.log
cat appium.log
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: E2E Tests

on: [push, pull_request]

jobs:
  desktop-e2e:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
      - uses: actions/checkout@v3

      - name: Setup Bun
        uses: oven-sh/setup-bun@v1

      - name: Install dependencies
        run: bun install

      - name: Build Tauri app
        run: bun run tauri:build

      - name: Run E2E tests
        run: bun run test:e2e

      - name: Upload screenshots on failure
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: test-screenshots
          path: e2e/screenshots/
```

## Troubleshooting

### tauri-driver Not Found

If you get "tauri-driver: command not found":

```bash
# Install tauri-driver
cargo install tauri-driver

# Verify installation
tauri-driver --version
```

Make sure `~/.cargo/bin` is in your PATH.

### Tests Can't Find the App Binary

The configuration automatically detects the binary path based on your platform. If it fails, set the `TAURI_BINARY` environment variable:

```bash
# macOS (note the bundle path)
export TAURI_BINARY="./src-tauri/target/debug/bundle/macos/tauri-app.app/Contents/MacOS/tauri-app"

# Linux
export TAURI_BINARY="./src-tauri/target/debug/tauri-app"

# Windows (PowerShell)
$env:TAURI_BINARY="./src-tauri/target/debug/tauri-app.exe"
```

### Build the App First

Make sure you've built the Tauri app before running tests:

```bash
bun run tauri:build:debug
```

### Android Tests Fail to Start

1. Verify emulator is running: `adb devices`
2. Check Appium is installed: `appium --version`
3. Verify APK exists: `ls src-tauri/gen/android/app/build/outputs/apk/`
4. Check Appium logs: `cat appium.log`

### Tests Are Flaky

1. Increase timeouts in `wdio.conf.ts`
2. Add explicit waits before assertions
3. Use `browser.pause()` strategically (but sparingly)
4. Ensure proper cleanup in `afterEach` hooks

### Element Not Found Errors

1. Verify the selector is correct
2. Wait for page to load: `await browser.pause(500)`
3. Use WebdriverIO's wait methods: `await element.waitForDisplayed()`
4. Check if element is in a shadow DOM or iframe

## Adding New Tests

1. **Create a new spec file** in `e2e/specs/`:
   ```typescript
   // e2e/specs/my-feature.spec.ts
   import { expect } from '@wdio/globals'

   describe('My Feature', () => {
     it('should work', async () => {
       // Test implementation
     })
   })
   ```

2. **Create a Page Object** if needed:
   ```typescript
   // e2e/pageobjects/my-feature.page.ts
   class MyFeaturePage {
     get myButton() {
       return $('button#my-button')
     }

     async clickMyButton() {
       await this.myButton.click()
     }
   }

   export default new MyFeaturePage()
   ```

3. **Run your new tests**:
   ```bash
   bun run test:e2e --spec ./e2e/specs/my-feature.spec.ts
   ```

## Resources

- [WebdriverIO Documentation](https://webdriver.io/docs/gettingstarted)
- [Tauri Testing Guide](https://tauri.app/v1/guides/testing/webdriver/introduction)
- [Mocha Documentation](https://mochajs.org/)
- [Page Object Pattern](https://martinfowler.com/bliki/PageObject.html)

## Support

For issues with the E2E tests:
1. Check the troubleshooting section above
2. Review test logs and screenshots
3. Consult WebdriverIO and Tauri documentation
4. Open an issue with reproduction steps

---

Happy Testing! 🧪
