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
    // Navigate to filesystem page
    const filesystemLink = await $('a[href="/filesystem"]')
    await filesystemLink.click()
    await browser.pause(500)

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
      const exists = await button.isExisting()
      expect(exists).toBe(true)
    }
  })

  it('should have input fields with default values', async () => {
    // Navigate to filesystem page
    const filesystemLink = await $('a[href="/filesystem"]')
    await filesystemLink.click()
    await browser.pause(500)

    // Check folder name input
    const folderInput = await $('input[value="test-folder"]')
    const folderExists = await folderInput.isExisting()
    expect(folderExists).toBe(true)

    // Check file name input
    const fileInput = await $('input[value="sample.txt"]')
    const fileExists = await fileInput.isExisting()
    expect(fileExists).toBe(true)

    // Check content textarea
    const textarea = await $('textarea')
    const textareaExists = await textarea.isExisting()
    expect(textareaExists).toBe(true)
  })
})
