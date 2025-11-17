import { expect } from '@wdio/globals'
import { waitForAppReady } from '../helpers/test-helpers.js'

describe('Tauri App Smoke Tests', () => {
  before(async () => {
    // Wait for Tauri app to be ready
    await waitForAppReady()
  })

  it('should launch the application successfully', async () => {
    // Verify the app window is present
    const title = await browser.getTitle()
    expect(title).toBeTruthy()

    // Verify body is rendered
    const body = await $('body')
    const isDisplayed = await body.isDisplayed()
    expect(isDisplayed).toBe(true)
  })

  it('should display the navigation menu', async () => {
    // Check for navigation links
    const filesystemLink = await $('a[href="/filesystem"]')
    const isExisting = await filesystemLink.isExisting()
    expect(isExisting).toBe(true)
  })

  it('should navigate to filesystem page', async () => {
    // Click filesystem link
    const filesystemLink = await $('a[href="/filesystem"]')
    await filesystemLink.waitForClickable()
    await filesystemLink.click()

    // Wait for page to load
    await browser.pause(500)

    // Verify page title
    const pageTitle = await $('h1*=Filesystem')
    const isDisplayed = await pageTitle.isDisplayed()
    expect(isDisplayed).toBe(true)
  })

  it('should display all filesystem action buttons', async () => {
    // We're already on filesystem page from previous test, no need to navigate again

    // Check for all buttons
    const buttons = [
      'Create Folder',
      'Write File',
      'Read File',
      'List Directory',
      'Check Exists',
      'Delete File',
    ]

    for (const buttonText of buttons) {
      const button = await $(`button*=${buttonText}`)
      await button.waitForDisplayed({ timeout: 5000 })
      const exists = await button.isExisting()
      expect(exists).toBe(true)
    }
  })

  it('should have input fields with default values', async () => {
    // We're already on filesystem page from previous tests, no need to navigate again

    // Check folder name input
    const folderInput = await $('[data-testid="folder-name-input"]')
    await folderInput.waitForDisplayed({ timeout: 5000 })
    const folderExists = await folderInput.isExisting()
    expect(folderExists).toBe(true)

    // Verify default value
    const folderValue = await folderInput.getValue()
    expect(folderValue).toBe('test-folder')

    // Check file name input
    const fileInput = await $('[data-testid="file-name-input"]')
    await fileInput.waitForDisplayed({ timeout: 5000 })
    const fileExists = await fileInput.isExisting()
    expect(fileExists).toBe(true)

    // Verify default value
    const fileValue = await fileInput.getValue()
    expect(fileValue).toBe('sample.txt')

    // Check content textarea
    const textarea = await $('[data-testid="file-content-textarea"]')
    await textarea.waitForDisplayed({ timeout: 5000 })
    const textareaExists = await textarea.isExisting()
    expect(textareaExists).toBe(true)

    // Verify default value
    const textareaValue = await textarea.getValue()
    expect(textareaValue).toBe('Hello from Tauri!')
  })
})
