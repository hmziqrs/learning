import type { Options } from '@wdio/types'

/**
 * WebdriverIO configuration for macOS Tauri E2E testing
 *
 * NOTE: tauri-driver doesn't fully support macOS yet.
 * This config tests the Tauri app in dev mode using Chrome DevTools Protocol
 *
 * Usage: bun run test:e2e:macos
 */
export const config: Options.Testrunner = {
  // Test specs
  specs: ['./e2e/specs/**/*.spec.ts'],

  // Exclude patterns
  exclude: [],

  // Maximum instances to run in parallel
  maxInstances: 1,

  // Use Chrome DevTools Protocol to connect to Tauri dev app
  automationProtocol: 'devtools',

  // Capabilities
  capabilities: [
    {
      browserName: 'chrome',
      'goog:chromeOptions': {
        debuggerAddress: 'localhost:9222', // Tauri dev mode debug port
      },
    },
  ],

  // Test Framework
  framework: 'mocha',
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000,
  },

  // Reporters
  reporters: ['spec'],

  // Logging
  logLevel: 'info',
  bail: 0,
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 3,

  // Services
  services: [],

  // Hooks
  before: async function () {
    console.log('🧪 Starting Tauri macOS E2E tests...')
    console.log('⚠️  Note: Testing via Chrome DevTools Protocol (tauri-driver not supported on macOS)')
    console.log('📱 Make sure Tauri dev server is running with: bun run tauri:dev')

    // Give the app a moment to initialize
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
    console.log('\n🎉 Tauri macOS E2E tests completed')
  },
}
