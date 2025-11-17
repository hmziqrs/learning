import type { Options } from '@wdio/types'
import { join } from 'path'
import { spawn, ChildProcess } from 'child_process'

let tauriDriver: ChildProcess | null = null

/**
 * WebdriverIO configuration for Tauri E2E testing
 * Uses tauri-driver to test the ACTUAL native Tauri application
 * This simulates real user interactions with the desktop app
 */
export const config: Options.Testrunner = {
  // Test specs
  specs: ['./e2e/specs/**/*.spec.ts'],

  // Exclude patterns
  exclude: [],

  // Maximum instances to run in parallel
  maxInstances: 1, // Keep at 1 for Tauri apps to avoid conflicts

  // Port for WebDriver connection
  port: 4444, // tauri-driver default port
  path: '/',

  // Force classic WebDriver protocol (not BiDi) to avoid unsupported capabilities
  automationProtocol: 'webdriver',

  // Capabilities for Tauri native app
  capabilities: [
    {
      // Tell WebDriver we're testing a Tauri app
      'tauri:options': {
        // Path to the actual Tauri binary that will be launched
        application: process.env.TAURI_BINARY || getBinaryPath(),
      },
      // Platform-specific capabilities (automatically detected)
      browserName: 'wry', // Tauri's webview name
      platformName: process.platform === 'darwin' ? 'mac' : process.platform === 'win32' ? 'windows' : 'linux',
      // Explicitly disable BiDi capabilities that tauri-driver doesn't support
      'wdio:enforceWebDriverClassic': true,
    } as any,
  ],

  // Test Framework
  framework: 'mocha',
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000, // 60 seconds for E2E tests
  },

  // Reporters
  reporters: ['spec'],

  // Logging
  logLevel: 'info',
  bail: 0,
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 3,

  // Services - we'll manually start tauri-driver in onPrepare
  services: [],

  // Transform requests to remove BiDi capabilities that tauri-driver doesn't support
  transformRequest: (requestOptions) => {
    // Remove webSocketUrl and unhandledPromptBehavior from capabilities
    if (requestOptions.body && typeof requestOptions.body === 'string') {
      try {
        const body = JSON.parse(requestOptions.body)
        if (body.capabilities?.alwaysMatch) {
          delete body.capabilities.alwaysMatch.webSocketUrl
          delete body.capabilities.alwaysMatch.unhandledPromptBehavior
          delete body.capabilities.alwaysMatch['wdio:enforceWebDriverClassic']
          requestOptions.body = JSON.stringify(body)
        }
      } catch (e) {
        // Ignore JSON parse errors
      }
    }
    return requestOptions
  },

  // Hooks
  onPrepare: async function () {
    // Start tauri-driver before tests
    const binaryPath = process.env.TAURI_BINARY || getBinaryPath()
    console.log('🚀 Starting tauri-driver...')
    console.log(`📦 Testing binary: ${binaryPath}`)

    // Ensure DISPLAY is set for Linux (Xvfb)
    if (process.platform === 'linux' && !process.env.DISPLAY) {
      process.env.DISPLAY = ':99'
      console.log('📺 Set DISPLAY=:99 for Xvfb')
    }

    // Start tauri-driver as a background process
    tauriDriver = spawn('tauri-driver', ['--port', '4444'], {
      stdio: 'inherit',
      env: process.env,
    })

    // Give tauri-driver time to start
    await new Promise((resolve) => setTimeout(resolve, 3000))

    console.log('✅ tauri-driver started on port 4444')
  },

  onComplete: async function () {
    // Stop tauri-driver after all tests
    if (tauriDriver) {
      console.log('🛑 Stopping tauri-driver...')
      tauriDriver.kill()
      tauriDriver = null
    }
  },

  before: async function () {
    // Setup before all tests
    console.log('🧪 Starting Tauri E2E tests...')
    console.log('📱 Testing native desktop application')

    // Give the app a moment to initialize after launch
    await browser.pause(2000)
  },

  beforeTest: async function (test, context) {
    console.log(`\n▶️  Running: ${test.title}`)
  },

  afterTest: async function (test, context, { error, result, duration, passed, retries }) {
    if (error) {
      console.error(`❌ Test failed: ${test.title}`)
      console.error(`   Error: ${error.message}`)

      // Take screenshot on failure
      try {
        const timestamp = new Date().toISOString().replace(/[:.]/g, '-')
        const filename = `e2e/screenshots/failed-${test.title.replace(/\s+/g, '-')}-${timestamp}.png`
        await browser.saveScreenshot(filename)
        console.log(`📸 Screenshot saved: ${filename}`)
      } catch (screenshotError) {
        console.error('Failed to take screenshot:', screenshotError)
      }
    } else {
      console.log(`✅ Test passed: ${test.title} (${duration}ms)`)
    }
  },

  after: async function () {
    console.log('\n🎉 Tauri E2E tests completed')
  },
}

/**
 * Get the binary path based on the platform
 * Returns the path to the actual native Tauri executable
 */
function getBinaryPath(): string {
  const platform = process.platform
  const projectRoot = process.cwd()

  // For development, we typically test the debug build
  // For CI/CD, use the release build by setting BUILD_TYPE=release
  const buildType = process.env.BUILD_TYPE || 'debug'

  let binaryPath: string

  switch (platform) {
    case 'darwin': // macOS
      binaryPath = join(
        projectRoot,
        'src-tauri/target',
        buildType,
        'bundle/macos/tauri-app.app/Contents/MacOS/tauri-app'
      )
      break
    case 'linux':
      binaryPath = join(projectRoot, 'src-tauri/target', buildType, 'tauri-app')
      break
    case 'win32': // Windows
      binaryPath = join(projectRoot, 'src-tauri/target', buildType, 'tauri-app.exe')
      break
    default:
      throw new Error(`Unsupported platform: ${platform}`)
  }

  console.log(`🔍 Binary path: ${binaryPath}`)
  return binaryPath
}
