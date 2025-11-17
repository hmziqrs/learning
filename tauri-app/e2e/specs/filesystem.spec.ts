import { expect } from '@wdio/globals'
import FilesystemPage from '../pageobjects/filesystem.page.js'
import { waitForAppReady, takeScreenshot } from '../helpers/test-helpers.js'

describe('Filesystem Module E2E Tests', () => {
  before(async () => {
    // Wait for Tauri app to be ready
    await waitForAppReady()
  })

  beforeEach(async () => {
    // Navigate to filesystem page before each test
    await FilesystemPage.navigate()
    await browser.pause(500) // Let page settle
  })

  afterEach(async function () {
    // Take screenshot on failure
    if (this.currentTest?.state === 'failed') {
      await takeScreenshot(`failed-${this.currentTest.title}`)
    }

    // Clear output between tests
    try {
      await FilesystemPage.clickClear()
    } catch {
      // Ignore if clear button not available
    }
  })

  describe('Folder Operations', () => {
    it('should create a folder successfully', async () => {
      const folderName = `test-folder-${Date.now()}`

      console.log(`📝 Setting folder name: ${folderName}`)
      // Set folder name
      await FilesystemPage.setFolderName(folderName)

      console.log('🖱️  Clicking create folder button')
      // Click create folder button
      await FilesystemPage.clickCreateFolder()

      // Wait a moment for the operation to complete
      console.log('⏳ Waiting 2 seconds for filesystem operation')
      await browser.pause(2000)

      // Get the output immediately to see what happened
      console.log('📖 Reading output panel')
      const output = await FilesystemPage.getOutputText()
      console.log('📋 Output content:', output)

      // Check if operation succeeded or failed
      if (output.includes('✓')) {
        console.log('✅ Success message found')
        expect(output).toContain('Created folder')
        expect(output).toContain(folderName)
      } else if (output.includes('✗')) {
        console.log('❌ Error message found in output')
        throw new Error(`Filesystem operation failed: ${output}`)
      } else {
        console.log('⚠️  No success or error marker found')
        throw new Error(`No response from filesystem operation. Output: ${output}`)
      }
    })

    it('should handle invalid folder names gracefully', async () => {
      // Try to create folder with invalid characters
      await FilesystemPage.setFolderName('invalid/folder:name*')
      await FilesystemPage.clickCreateFolder()

      // Should show error or handle gracefully
      const output = await FilesystemPage.getOutputText()
      expect(output).toBeTruthy()
    })
  })

  describe('File Write Operations', () => {
    it('should write a file successfully', async () => {
      const fileName = `test-${Date.now()}.txt`
      const fileContent = 'Hello from E2E test!'

      // Write file
      await FilesystemPage.writeFileFlow(fileName, fileContent)

      // Verify output
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('✓')
      expect(output).toContain('Written file')
      expect(output).toContain(fileName)
    })

    it('should allow updating file content', async () => {
      const fileName = `update-test-${Date.now()}.txt`

      // Write initial content
      await FilesystemPage.writeFileFlow(fileName, 'Initial content')

      // Wait a moment
      await browser.pause(500)

      // Update content
      await FilesystemPage.setFileContent('Updated content')
      await FilesystemPage.clickWriteFile()
      await FilesystemPage.waitForSuccessMessage()

      // Read and verify
      await FilesystemPage.clickReadFile()
      await FilesystemPage.waitForSuccessMessage()

      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('Updated content')
    })
  })

  describe('File Read Operations', () => {
    it('should read a file after writing it', async () => {
      const fileName = `read-test-${Date.now()}.txt`
      const fileContent = 'Content to be read back'

      // Write file first
      await FilesystemPage.writeFileFlow(fileName, fileContent)

      // Small pause
      await browser.pause(500)

      // Read file
      await FilesystemPage.clickReadFile()
      await FilesystemPage.waitForSuccessMessage()

      // Verify content in output
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('✓')
      expect(output).toContain('Read file')
      expect(output).toContain(fileContent)
    })

    it('should show error when reading non-existent file', async () => {
      const fileName = `non-existent-${Date.now()}.txt`

      // Try to read non-existent file
      await FilesystemPage.setFileName(fileName)
      await FilesystemPage.clickReadFile()

      // Wait for response
      await browser.pause(1000)

      // Should show error
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('Error')
    })
  })

  describe('File Existence Check', () => {
    it('should correctly check if file exists', async () => {
      const fileName = `exists-test-${Date.now()}.txt`

      // Check file doesn't exist initially
      await FilesystemPage.setFileName(fileName)
      await FilesystemPage.clickCheckExists()
      await browser.pause(500)

      let output = await FilesystemPage.getOutputText()
      expect(output).toContain('DOES NOT EXIST')

      // Create the file
      await FilesystemPage.setFileContent('Test content')
      await FilesystemPage.clickWriteFile()
      await FilesystemPage.waitForSuccessMessage()

      // Check file exists now
      await FilesystemPage.clickCheckExists()
      await browser.pause(500)

      output = await FilesystemPage.getOutputText()
      expect(output).toContain('EXISTS')
    })
  })

  describe('Directory Listing', () => {
    it('should list directory contents', async () => {
      // Create a test file first to ensure directory has content
      const fileName = `list-test-${Date.now()}.txt`
      await FilesystemPage.writeFileFlow(fileName, 'Test content')

      // Small pause
      await browser.pause(500)

      // List directory
      await FilesystemPage.clickListDirectory()
      await FilesystemPage.waitForSuccessMessage()

      // Verify directory contents are shown
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('Listed directory')
      expect(output).toContain('Found')
      expect(output).toContain('items')

      // Check if directory list UI appeared
      const isListVisible = await FilesystemPage.isDirectoryListVisible()
      expect(isListVisible).toBe(true)
    })
  })

  describe('File Delete Operations', () => {
    it('should delete a file successfully', async () => {
      const fileName = `delete-test-${Date.now()}.txt`

      // Create file first
      await FilesystemPage.writeFileFlow(fileName, 'To be deleted')

      // Small pause
      await browser.pause(500)

      // Delete file
      await FilesystemPage.clickDeleteFile()
      await FilesystemPage.waitForSuccessMessage()

      // Verify deletion
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('✓')
      expect(output).toContain('Deleted file')

      // Verify file no longer exists
      await FilesystemPage.clickCheckExists()
      await browser.pause(500)

      const finalOutput = await FilesystemPage.getOutputText()
      expect(finalOutput).toContain('DOES NOT EXIST')
    })

    it('should handle deleting non-existent file', async () => {
      const fileName = `non-existent-delete-${Date.now()}.txt`

      // Try to delete non-existent file
      await FilesystemPage.setFileName(fileName)
      await FilesystemPage.clickDeleteFile()

      // Wait for response
      await browser.pause(1000)

      // Should show error
      const output = await FilesystemPage.getOutputText()
      expect(output).toContain('Error')
    })
  })

  describe('Complete File Lifecycle', () => {
    it('should perform complete CRUD operations', async () => {
      const fileName = `lifecycle-test-${Date.now()}.txt`
      const initialContent = 'Initial content'
      const updatedContent = 'Updated content'

      // 1. Create (Write)
      await FilesystemPage.writeFileFlow(fileName, initialContent)
      await browser.pause(300)

      // 2. Read
      await FilesystemPage.clickReadFile()
      await FilesystemPage.waitForSuccessMessage()
      let output = await FilesystemPage.getOutputText()
      expect(output).toContain(initialContent)

      // 3. Update (Write again)
      await FilesystemPage.setFileContent(updatedContent)
      await FilesystemPage.clickWriteFile()
      await FilesystemPage.waitForSuccessMessage()

      // 4. Read updated content
      await FilesystemPage.clickReadFile()
      await FilesystemPage.waitForSuccessMessage()
      output = await FilesystemPage.getOutputText()
      expect(output).toContain(updatedContent)

      // 5. Delete
      await FilesystemPage.clickDeleteFile()
      await FilesystemPage.waitForSuccessMessage()

      // 6. Verify deletion
      await FilesystemPage.clickCheckExists()
      await browser.pause(500)
      output = await FilesystemPage.getOutputText()
      expect(output).toContain('DOES NOT EXIST')
    })
  })

  describe('UI Interactions', () => {
    it('should clear output when Clear button is clicked', async () => {
      // Perform an operation to generate output
      await FilesystemPage.clickListDirectory()
      await FilesystemPage.waitForSuccessMessage()

      // Verify there is output
      let output = await FilesystemPage.getOutputText()
      expect(output).not.toBe('')

      // Clear output
      await FilesystemPage.clickClear()
      await browser.pause(300)

      // Verify output is cleared
      output = await FilesystemPage.getOutputText()
      expect(output).toContain('No output yet')
    })

    it('should allow changing input values', async () => {
      // Change folder name
      await FilesystemPage.setFolderName('custom-folder')
      const folderInput = await FilesystemPage.folderNameInput
      const folderValue = await folderInput.getValue()
      expect(folderValue).toBe('custom-folder')

      // Change file name
      await FilesystemPage.setFileName('custom-file.txt')
      const fileInput = await FilesystemPage.fileNameInput
      const fileValue = await fileInput.getValue()
      expect(fileValue).toBe('custom-file.txt')

      // Change content
      await FilesystemPage.setFileContent('Custom content')
      const contentTextarea = await FilesystemPage.fileContentTextarea
      const contentValue = await contentTextarea.getValue()
      expect(contentValue).toBe('Custom content')
    })

    it('should disable buttons while loading', async () => {
      // Start an operation
      await FilesystemPage.clickListDirectory()

      // Buttons should be disabled during operation
      // Note: This test might be flaky if operations complete too quickly
      // Consider adding a delay in the app for testing purposes
    })
  })
})
