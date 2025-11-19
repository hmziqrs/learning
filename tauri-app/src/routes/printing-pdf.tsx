import { createFileRoute } from '@tanstack/react-router'
import { FileText, Printer, File, Files, Upload, Download, Eye } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { useState, useRef } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import { readFile, writeFile } from '@tauri-apps/plugin-fs'
import jsPDF from 'jspdf'
import { Document, Page, pdfjs } from 'react-pdf'
import 'react-pdf/dist/Page/AnnotationLayer.css'
import 'react-pdf/dist/Page/TextLayer.css'

// Set up PDF.js worker
pdfjs.GlobalWorkerOptions.workerSrc = `//unpkg.com/pdfjs-dist@${pdfjs.version}/build/pdf.worker.min.mjs`

export const Route = createFileRoute('/printing-pdf')({
  component: PrintingPdfModule,
})

interface PdfMetadata {
  title: string
  author: string
  subject: string
  creator: string
  producer: string
  pageCount: number
  fileSize: number
}

function PrintingPdfModule() {
  const [output, setOutput] = useState<string[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [pdfContent, setPdfContent] = useState('<h1>Sample Document</h1><p>This is a test PDF document with some content.</p><p>You can edit this HTML to customize your PDF.</p>')
  const [pdfTitle, setPdfTitle] = useState('Test Document')
  const [pdfAuthor, setPdfAuthor] = useState('Tauri App')
  const [pdfUrl, setPdfUrl] = useState<string | null>(null)
  const [numPages, setNumPages] = useState<number>(0)
  const [pageNumber, setPageNumber] = useState<number>(1)
  const [currentPdfPath, setCurrentPdfPath] = useState<string | null>(null)
  const [mergeFiles, setMergeFiles] = useState<string[]>([])
  const fileInputRef = useRef<HTMLInputElement>(null)

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Generate PDF using jsPDF
  const handleGeneratePdfJs = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Generating PDF with jsPDF...')

    try {
      // Create new PDF document
      const doc = new jsPDF({
        orientation: 'portrait',
        unit: 'mm',
        format: 'a4',
      })

      // Set document properties
      doc.setProperties({
        title: pdfTitle,
        author: pdfAuthor,
        subject: 'Generated PDF',
        creator: 'Tauri PDF Module',
      })

      // Parse HTML content and add to PDF
      // For better HTML rendering, we'll use html method
      const element = document.createElement('div')
      element.innerHTML = pdfContent
      element.style.width = '180mm'
      element.style.padding = '10mm'
      element.style.fontFamily = 'Arial, sans-serif'

      await doc.html(element, {
        callback: async (doc) => {
          // Get PDF as blob
          const pdfBlob = doc.output('blob')

          // Ask user where to save
          const filePath = await save({
            defaultPath: `${pdfTitle}.pdf`,
            filters: [{ name: 'PDF', extensions: ['pdf'] }],
          })

          if (filePath) {
            // Convert blob to array buffer
            const arrayBuffer = await pdfBlob.arrayBuffer()
            const uint8Array = new Uint8Array(arrayBuffer)

            // Write file using Tauri
            await writeFile(filePath, uint8Array)

            addOutput(`PDF generated and saved to: ${filePath}`)
            setCurrentPdfPath(filePath)

            // Create object URL for preview
            const url = URL.createObjectURL(pdfBlob)
            setPdfUrl(url)
            setPageNumber(1)
          } else {
            addOutput('PDF generation cancelled', false)
          }

          setIsLoading(false)
        },
        x: 10,
        y: 10,
        width: 180,
        windowWidth: 800,
      })
    } catch (error) {
      addOutput(`Failed to generate PDF: ${error}`, false)
      setIsLoading(false)
    }
  }

  // Generate simple text PDF
  const handleGenerateSimplePdf = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Generating simple text PDF...')

    try {
      const doc = new jsPDF()

      doc.setProperties({
        title: pdfTitle,
        author: pdfAuthor,
        subject: 'Simple PDF',
      })

      // Add title
      doc.setFontSize(20)
      doc.text(pdfTitle, 20, 20)

      // Add content (strip HTML tags for simple version)
      doc.setFontSize(12)
      const textContent = pdfContent.replace(/<[^>]*>/g, ' ').replace(/\s+/g, ' ').trim()
      const splitText = doc.splitTextToSize(textContent, 170)
      doc.text(splitText, 20, 40)

      // Get PDF as blob
      const pdfBlob = doc.output('blob')

      // Ask user where to save
      const filePath = await save({
        defaultPath: `${pdfTitle}.pdf`,
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      })

      if (filePath) {
        const arrayBuffer = await pdfBlob.arrayBuffer()
        const uint8Array = new Uint8Array(arrayBuffer)
        await writeFile(filePath, uint8Array)

        addOutput(`Simple PDF saved to: ${filePath}`)
        setCurrentPdfPath(filePath)

        // Create preview
        const url = URL.createObjectURL(pdfBlob)
        setPdfUrl(url)
        setPageNumber(1)
      }
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Generate PDF using browser print
  const handleGeneratePdfBrowser = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Opening browser print dialog...')

    try {
      const printWindow = window.open('', '', 'width=800,height=600')
      if (printWindow) {
        printWindow.document.write(`
          <!DOCTYPE html>
          <html>
            <head>
              <title>${pdfTitle}</title>
              <style>
                body { font-family: Arial, sans-serif; padding: 40px; max-width: 800px; margin: 0 auto; }
                h1 { color: #333; margin-bottom: 20px; }
                p { line-height: 1.6; margin-bottom: 10px; }
                @media print {
                  body { padding: 20px; }
                }
              </style>
            </head>
            <body>
              ${pdfContent}
            </body>
          </html>
        `)
        printWindow.document.close()
        printWindow.focus()

        setTimeout(() => {
          printWindow.print()
          addOutput('Print dialog opened - select "Save as PDF"')
        }, 250)
      } else {
        addOutput('Failed to open print window - popup may be blocked', false)
      }
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Load and display PDF
  const handleLoadPdf = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Opening file picker...')

    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      })

      if (filePath) {
        addOutput(`Loading PDF: ${filePath}`)

        // Read file
        const fileData = await readFile(filePath)

        // Create blob and object URL
        const blob = new Blob([fileData], { type: 'application/pdf' })
        const url = URL.createObjectURL(blob)

        setPdfUrl(url)
        setCurrentPdfPath(filePath)
        setPageNumber(1)

        addOutput(`PDF loaded successfully`)
      }
    } catch (error) {
      addOutput(`Failed to load PDF: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Get PDF info using Rust backend
  const handleGetPdfInfo = async () => {
    if (!currentPdfPath) {
      addOutput('Please load a PDF first', false)
      return
    }

    if (isLoading) return
    setIsLoading(true)
    addOutput('Getting PDF metadata...')

    try {
      const info = await invoke<PdfMetadata>('get_pdf_info', {
        filePath: currentPdfPath
      })

      addOutput(`Title: ${info.title}`)
      addOutput(`Author: ${info.author}`)
      addOutput(`Pages: ${info.pageCount}`)
      addOutput(`Size: ${(info.fileSize / 1024).toFixed(2)} KB`)
    } catch (error) {
      addOutput(`Command not implemented yet: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Merge PDFs
  const handleMergePdfs = async () => {
    if (mergeFiles.length < 2) {
      addOutput('Please select at least 2 PDF files to merge', false)
      return
    }

    if (isLoading) return
    setIsLoading(true)
    addOutput(`Merging ${mergeFiles.length} PDF files...`)

    try {
      const outputPath = await save({
        defaultPath: 'merged.pdf',
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      })

      if (outputPath) {
        await invoke('merge_pdfs', {
          inputPaths: mergeFiles,
          outputPath,
        })

        addOutput(`PDFs merged successfully: ${outputPath}`)
        setMergeFiles([])
      }
    } catch (error) {
      addOutput(`Merge failed (command not implemented): ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Add file to merge list
  const handleAddMergeFile = async () => {
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      })

      if (filePath) {
        setMergeFiles((prev) => [...prev, filePath])
        addOutput(`Added to merge list: ${filePath}`)
      }
    } catch (error) {
      addOutput(`Failed to add file: ${error}`, false)
    }
  }

  // PDF document loaded callback
  const onDocumentLoadSuccess = ({ numPages }: { numPages: number }) => {
    setNumPages(numPages)
    addOutput(`PDF loaded with ${numPages} pages`)
  }

  // Preview current content
  const handlePreviewPdf = () => {
    const previewWindow = window.open('', '_blank')
    if (previewWindow) {
      previewWindow.document.write(`
        <!DOCTYPE html>
        <html>
          <head>
            <title>${pdfTitle} - Preview</title>
            <style>
              body {
                font-family: Arial, sans-serif;
                max-width: 800px;
                margin: 0 auto;
                padding: 40px 20px;
                background: #f5f5f5;
              }
              .page {
                background: white;
                padding: 40px;
                box-shadow: 0 2px 8px rgba(0,0,0,0.1);
                min-height: 1000px;
              }
              h1 { color: #333; margin-top: 0; }
              p { line-height: 1.6; color: #666; }
            </style>
          </head>
          <body>
            <div class="page">
              ${pdfContent}
            </div>
          </body>
        </html>
      `)
      previewWindow.document.close()
      addOutput('Preview opened in new tab')
    }
  }

  return (
    <ModulePageLayout
      title="Printing & PDF / Document Handling"
      description="Generate PDFs, print documents, and handle document operations"
      icon={FileText}
    >
      <div className="space-y-6">
        {/* Status Notice */}
        <section className="rounded-lg border border-green-500/50 bg-green-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-green-500">✓</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Fully implemented features:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ jsPDF Integration</strong> - Client-side PDF generation
              </li>
              <li>
                <strong className="text-green-600">✓ react-pdf Viewer</strong> - PDF viewing and navigation
              </li>
              <li>
                <strong className="text-green-600">✓ Browser Print API</strong> - Native print dialogs
              </li>
              <li>
                <strong className="text-yellow-600">⚠ Rust Backend</strong> - Advanced operations require implementation
              </li>
            </ul>
          </div>
        </section>

        {/* PDF Content Editor */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <File className="w-5 h-5" />
            PDF Content Editor
          </h2>

          <div className="space-y-3">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div className="space-y-2">
                <label htmlFor="pdf-title" className="text-sm font-medium">
                  Document Title
                </label>
                <Input
                  id="pdf-title"
                  value={pdfTitle}
                  onChange={(e) => setPdfTitle(e.target.value)}
                  placeholder="Enter document title"
                />
              </div>

              <div className="space-y-2">
                <label htmlFor="pdf-author" className="text-sm font-medium">
                  Author
                </label>
                <Input
                  id="pdf-author"
                  value={pdfAuthor}
                  onChange={(e) => setPdfAuthor(e.target.value)}
                  placeholder="Enter author name"
                />
              </div>
            </div>

            <div className="space-y-2">
              <label htmlFor="pdf-content" className="text-sm font-medium">
                HTML Content
              </label>
              <Textarea
                id="pdf-content"
                value={pdfContent}
                onChange={(e) => setPdfContent(e.target.value)}
                placeholder="Enter HTML content for the PDF"
                className="w-full font-mono text-sm"
                rows={8}
              />
              <p className="text-xs text-muted-foreground">
                Supports HTML tags: h1-h6, p, strong, em, ul, ol, li, div, etc.
              </p>
            </div>

            <div className="flex flex-wrap gap-2">
              <Button onClick={handlePreviewPdf} variant="outline" size="sm">
                <Eye className="w-4 h-4 mr-2" />
                Preview
              </Button>
            </div>
          </div>
        </section>

        {/* PDF Generation */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <FileText className="w-5 h-5" />
            Generate PDF
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Create PDF documents from your content using different methods
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">HTML to PDF</h3>
                  <p className="text-xs text-muted-foreground">
                    Full HTML rendering with styles
                  </p>
                </div>
                <Button
                  onClick={handleGeneratePdfJs}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <FileText className="w-4 h-4 mr-2" />
                  Generate (jsPDF)
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">Simple Text PDF</h3>
                  <p className="text-xs text-muted-foreground">
                    Plain text without HTML
                  </p>
                </div>
                <Button
                  onClick={handleGenerateSimplePdf}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <FileText className="w-4 h-4 mr-2" />
                  Simple PDF
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">Browser Print</h3>
                  <p className="text-xs text-muted-foreground">
                    Use system print dialog
                  </p>
                </div>
                <Button
                  onClick={handleGeneratePdfBrowser}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <Printer className="w-4 h-4 mr-2" />
                  Print Dialog
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* PDF Viewer */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Eye className="w-5 h-5" />
            PDF Viewer
          </h2>

          <div className="space-y-3">
            <div className="flex flex-wrap gap-2">
              <Button onClick={handleLoadPdf} variant="outline" disabled={isLoading}>
                <Upload className="w-4 h-4 mr-2" />
                Load PDF
              </Button>

              {currentPdfPath && (
                <Button onClick={handleGetPdfInfo} variant="outline" disabled={isLoading}>
                  <FileText className="w-4 h-4 mr-2" />
                  Get Info
                </Button>
              )}
            </div>

            {pdfUrl && (
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setPageNumber((prev) => Math.max(1, prev - 1))}
                      disabled={pageNumber <= 1}
                    >
                      Previous
                    </Button>
                    <span className="text-sm">
                      Page {pageNumber} of {numPages}
                    </span>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setPageNumber((prev) => Math.min(numPages, prev + 1))}
                      disabled={pageNumber >= numPages}
                    >
                      Next
                    </Button>
                  </div>
                </div>

                <div className="border rounded-lg p-4 bg-muted/30 overflow-auto max-h-[600px]">
                  <Document
                    file={pdfUrl}
                    onLoadSuccess={onDocumentLoadSuccess}
                    loading={
                      <div className="flex items-center justify-center p-8">
                        <p className="text-muted-foreground">Loading PDF...</p>
                      </div>
                    }
                    error={
                      <div className="flex items-center justify-center p-8">
                        <p className="text-red-500">Failed to load PDF</p>
                      </div>
                    }
                  >
                    <Page
                      pageNumber={pageNumber}
                      renderTextLayer={true}
                      renderAnnotationLayer={true}
                      className="mx-auto"
                    />
                  </Document>
                </div>
              </div>
            )}

            {!pdfUrl && (
              <div className="border rounded-lg p-8 text-center text-muted-foreground">
                <FileText className="w-12 h-12 mx-auto mb-3 opacity-50" />
                <p>No PDF loaded. Generate or load a PDF to preview it here.</p>
              </div>
            )}
          </div>
        </section>

        {/* Advanced PDF Operations */}
        <section className="rounded-lg border p-6 space-y-4 border-purple-500/30 bg-purple-500/5">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Files className="w-5 h-5" />
            Advanced PDF Operations
          </h2>

          <div className="space-y-4">
            <div className="space-y-3">
              <h3 className="font-semibold text-sm">Merge PDFs</h3>
              <p className="text-xs text-muted-foreground">
                Combine multiple PDF files into one (requires Rust backend)
              </p>

              <div className="flex flex-wrap gap-2">
                <Button onClick={handleAddMergeFile} variant="outline" size="sm">
                  <Upload className="w-4 h-4 mr-2" />
                  Add PDF to Merge
                </Button>

                {mergeFiles.length > 0 && (
                  <Button onClick={handleMergePdfs} variant="outline" size="sm">
                    <Files className="w-4 h-4 mr-2" />
                    Merge {mergeFiles.length} Files
                  </Button>
                )}
              </div>

              {mergeFiles.length > 0 && (
                <div className="bg-muted rounded-md p-3 text-xs font-mono space-y-1">
                  {mergeFiles.map((file, i) => (
                    <div key={i} className="flex items-center justify-between">
                      <span>{file}</span>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setMergeFiles((prev) => prev.filter((_, idx) => idx !== i))}
                      >
                        Remove
                      </Button>
                    </div>
                  ))}
                </div>
              )}
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <p className="text-sm text-yellow-700 dark:text-yellow-400">
                <strong>Note:</strong> Advanced operations (merge, split, extract text, encryption) require Rust backend implementation.
                These features will invoke Tauri commands when the backend is ready.
              </p>
            </div>
          </div>
        </section>

        {/* Output Panel */}
        <section className="rounded-lg border p-6 space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold">Output</h2>
            <Button variant="outline" size="sm" onClick={() => setOutput([])}>
              Clear
            </Button>
          </div>

          <div className="bg-muted rounded-md p-4 h-64 overflow-y-auto font-mono text-sm">
            {output.length === 0 ? (
              <p className="text-muted-foreground">No output yet. Try generating or loading a PDF...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
