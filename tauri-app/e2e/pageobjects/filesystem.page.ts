import { waitAndClick, waitAndSetValue, waitForText } from '../helpers/test-helpers.js'

/**
 * Page Object for Filesystem module
 * Encapsulates all interactions with the filesystem page
 */
class FilesystemPage {
  // Selectors
  get folderNameInput() {
    return $('[data-testid="folder-name-input"]')
  }

  get fileNameInput() {
    return $('[data-testid="file-name-input"]')
  }

  get fileContentTextarea() {
    return $('[data-testid="file-content-textarea"]')
  }

  get createFolderButton() {
    return $('button*=Create Folder')
  }

  get writeFileButton() {
    return $('button*=Write File')
  }

  get readFileButton() {
    return $('button*=Read File')
  }

  get listDirectoryButton() {
    return $('button*=List Directory')
  }

  get checkExistsButton() {
    return $('button*=Check Exists')
  }

  get deleteFileButton() {
    return $('button*=Delete File')
  }

  get clearButton() {
    return $('button*=Clear')
  }

  get outputPanel() {
    return $('pre')
  }

  get directoryContents() {
    return $('h3*=Directory Contents')
  }

  // Actions
  async navigate() {
    // Click on Filesystem link in navigation
    await waitAndClick('a[href="/filesystem"]')
    await browser.pause(500)
  }

  async setFolderName(name: string) {
    const input = await this.folderNameInput
    await input.waitForDisplayed()
    await input.scrollIntoView()
    await browser.pause(200) // Small pause after scroll
    await input.clearValue()
    await input.setValue(name)
  }

  async setFileName(name: string) {
    const input = await this.fileNameInput
    await input.waitForDisplayed()
    await input.clearValue()
    await input.setValue(name)
  }

  async setFileContent(content: string) {
    const textarea = await this.fileContentTextarea
    await textarea.waitForDisplayed()
    await textarea.clearValue()
    await textarea.setValue(content)
  }

  async clickCreateFolder() {
    const button = await this.createFolderButton
    await button.waitForClickable()
    await button.scrollIntoView()
    await browser.pause(200) // Small pause after scroll
    await button.click()
  }

  async clickWriteFile() {
    const button = await this.writeFileButton
    await button.waitForClickable()
    await button.click()
  }

  async clickReadFile() {
    const button = await this.readFileButton
    await button.waitForClickable()
    await button.click()
  }

  async clickListDirectory() {
    const button = await this.listDirectoryButton
    await button.waitForClickable()
    await button.click()
  }

  async clickCheckExists() {
    const button = await this.checkExistsButton
    await button.waitForClickable()
    await button.click()
  }

  async clickDeleteFile() {
    const button = await this.deleteFileButton
    await button.waitForClickable()
    await button.click()
  }

  async clickClear() {
    const button = await this.clearButton
    await button.waitForClickable()
    await button.click()
  }

  async getOutputText(): Promise<string> {
    const output = await this.outputPanel
    await output.waitForDisplayed()
    return await output.getText()
  }

  async waitForSuccessMessage(timeout = 10000) {
    await waitForText('✓', timeout)
  }

  async waitForErrorMessage(timeout = 10000) {
    await waitForText('✗', timeout)
  }

  async isDirectoryListVisible(): Promise<boolean> {
    const element = await this.directoryContents
    return await element.isDisplayed()
  }

  // Composite actions (common workflows)
  async createFolderFlow(folderName: string) {
    await this.setFolderName(folderName)
    await this.clickCreateFolder()
    await this.waitForSuccessMessage()
  }

  async writeFileFlow(fileName: string, content: string) {
    await this.setFileName(fileName)
    await this.setFileContent(content)
    await this.clickWriteFile()
    await this.waitForSuccessMessage()
  }

  async readFileFlow(fileName: string) {
    await this.setFileName(fileName)
    await this.clickReadFile()
    await this.waitForSuccessMessage()
  }

  async deleteFileFlow(fileName: string) {
    await this.setFileName(fileName)
    await this.clickDeleteFile()
    await this.waitForSuccessMessage()
  }
}

export default new FilesystemPage()
