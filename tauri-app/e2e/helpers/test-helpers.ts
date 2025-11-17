/**
 * Test helper utilities for Tauri E2E tests
 */

/**
 * Wait for an element to be displayed and clickable
 */
export async function waitAndClick(selector: string, timeout = 10000) {
  const element = await $(selector)
  await element.waitForDisplayed({ timeout })
  await element.waitForClickable({ timeout })
  await element.click()
}

/**
 * Wait for an element and set its value
 */
export async function waitAndSetValue(selector: string, value: string, timeout = 10000) {
  const element = await $(selector)
  await element.waitForDisplayed({ timeout })
  await element.setValue(value)
}

/**
 * Wait for an element and get its text
 */
export async function waitAndGetText(selector: string, timeout = 10000): Promise<string> {
  const element = await $(selector)
  await element.waitForDisplayed({ timeout })
  return await element.getText()
}

/**
 * Wait for text to appear anywhere on the page
 */
export async function waitForText(text: string | RegExp, timeout = 10000) {
  const selector = typeof text === 'string' ? `*=${text}` : text
  const element = await $(selector)
  await element.waitForDisplayed({ timeout })
}

/**
 * Check if element exists without throwing
 */
export async function elementExists(selector: string): Promise<boolean> {
  try {
    const element = await $(selector)
    return await element.isExisting()
  } catch {
    return false
  }
}

/**
 * Wait for the app to be ready
 * Tauri apps may take a moment to initialize
 */
export async function waitForAppReady(timeout = 15000) {
  await browser.pause(2000) // Initial wait for webview

  // Wait for body to be present
  const body = await $('body')
  await body.waitForDisplayed({ timeout })

  // Additional wait for React hydration
  await browser.pause(1000)
}

/**
 * Take a screenshot for debugging
 */
export async function takeScreenshot(name: string) {
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-')
  const filename = `${name}-${timestamp}.png`
  await browser.saveScreenshot(`./e2e/screenshots/${filename}`)
  console.log(`📸 Screenshot saved: ${filename}`)
}

/**
 * Clear all input fields by selector
 */
export async function clearInput(selector: string) {
  const element = await $(selector)
  await element.waitForDisplayed()
  await element.clearValue()
}

/**
 * Get all text from an element including children
 */
export async function getAllText(selector: string): Promise<string> {
  const element = await $(selector)
  await element.waitForDisplayed()
  return await element.getText()
}
