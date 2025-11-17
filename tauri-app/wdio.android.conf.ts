import type { Options } from '@wdio/types'
import { config as baseConfig } from './wdio.conf.js'
import { join } from 'path'

/**
 * WebdriverIO configuration for Android E2E testing
 * Requires Appium server to be running
 */
export const config: Options.Testrunner = {
  ...baseConfig,

  // Override port for Appium
  port: 4723,
  path: '/',

  // Android-specific capabilities
  capabilities: [
    {
      platformName: 'Android',
      'appium:automationName': 'UiAutomator2',
      'appium:deviceName': 'Android Emulator',
      'appium:platformVersion': '15.0', // Adjust based on your emulator
      'appium:app': getApkPath(),
      'appium:appPackage': 'com.tauri_app.app', // Update with your package name
      'appium:appActivity': '.MainActivity',
      'appium:noReset': false,
      'appium:fullReset': false,
      'appium:newCommandTimeout': 240,
    },
  ],

  // Android-specific services
  services: [
    [
      'appium',
      {
        command: 'appium',
        args: {
          relaxedSecurity: true,
          log: './appium.log',
        },
      },
    ],
  ],

  // Increase timeouts for Android
  waitforTimeout: 15000,
  connectionRetryTimeout: 180000,

  // Hooks
  before: async function () {
    console.log('🤖 Starting Android E2E tests...')
    // Wait for app to initialize
    await browser.pause(3000)
  },

  after: async function () {
    console.log('✅ Android E2E tests completed')
  },
}

/**
 * Get the APK path for testing
 */
function getApkPath(): string {
  const projectRoot = process.cwd()
  const buildType = process.env.BUILD_TYPE || 'debug'

  if (buildType === 'release') {
    return join(
      projectRoot,
      'src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk'
    )
  }

  return join(
    projectRoot,
    'src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk'
  )
}
