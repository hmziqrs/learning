import { createFileRoute } from '@tanstack/react-router'
import { FileText, Printer, File, Files } from 'lucide-react'
import { ModulePageLayout } from '@/components/module-page-layout'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { useState } from 'react'

export const Route = createFileRoute('/printing-pdf')({
  component: PrintingPdfModule,
})

function PrintingPdfModule() {
  const [output, setOutput] = useState<string[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [pdfContent, setPdfContent] = useState('<h1>Sample Document</h1><p>This is a test PDF document.</p>')
  const [pdfTitle, setPdfTitle] = useState('Test Document')

  const addOutput = (message: string, success: boolean = true) => {
    const icon = success ? '✓' : '✗'
    const timestamp = new Date().toLocaleTimeString()
    setOutput((prev) => [...prev, `[${timestamp}] ${icon} ${message}`])
  }

  // Generate PDF using browser print
  const handleGeneratePdfBrowser = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Generating PDF using browser print API...')

    try {
      // Create a temporary window with the content
      const printWindow = window.open('', '', 'width=800,height=600')
      if (printWindow) {
        printWindow.document.write(`
          <!DOCTYPE html>
          <html>
            <head>
              <title>${pdfTitle}</title>
              <style>
                body { font-family: Arial, sans-serif; padding: 20px; }
                h1 { color: #333; }
                p { line-height: 1.6; }
              </style>
            </head>
            <body>
              ${pdfContent}
            </body>
          </html>
        `)
        printWindow.document.close()
        printWindow.focus()

        // Wait for content to load, then trigger print
        setTimeout(() => {
          printWindow.print()
          addOutput('Browser print dialog opened successfully')
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

  // Print current page
  const handlePrintCurrentPage = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Opening print dialog for current page...')

    try {
      window.print()
      addOutput('Print dialog opened successfully')
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Generate PDF with jsPDF (requires installation)
  const handleGeneratePdfJs = async () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Note: jsPDF library required for this feature')
    addOutput('Install with: bun add jspdf')

    try {
      // This will only work if jsPDF is installed
      // const { jsPDF } = await import('jspdf')
      // const doc = new jsPDF()
      // doc.text(pdfContent, 10, 10)
      // doc.save(`${pdfTitle}.pdf`)

      addOutput('jsPDF functionality will be available after library installation', false)
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
    }
  }

  // Preview PDF content
  const handlePreviewPdf = () => {
    if (isLoading) return
    setIsLoading(true)
    addOutput('Generating PDF preview...')

    try {
      // Create preview in new tab
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
        addOutput('PDF preview opened in new tab')
      } else {
        addOutput('Failed to open preview window - popup may be blocked', false)
      }
    } catch (error) {
      addOutput(`Failed: ${error}`, false)
    } finally {
      setIsLoading(false)
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
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/10 p-6">
          <h3 className="text-lg font-semibold mb-2 flex items-center gap-2">
            <span className="text-blue-500">ℹ️</span>
            Implementation Status
          </h3>
          <div className="space-y-2 text-sm">
            <p className="font-medium">Current implementation:</p>
            <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
              <li>
                <strong className="text-green-600">✓ Browser Print API</strong> - Native window.print() available
              </li>
              <li>
                <strong className="text-yellow-600">⚠ jsPDF</strong> - Requires installation: bun add jspdf
              </li>
              <li>
                <strong className="text-yellow-600">⚠ react-pdf</strong> - Requires installation: bun add react-pdf
              </li>
              <li>
                <strong className="text-red-600">✗ Rust Backend</strong> - Advanced PDF operations pending implementation
              </li>
            </ul>
            <div className="bg-muted rounded-md p-3 font-mono text-xs mt-2">
              <div># Browser-Based Implementation (Available):</div>
              <div>window.print() - System print dialog</div>
              <div>HTML to PDF - Browser print to PDF</div>
              <div className="mt-2"># Advanced Features (Requires Setup):</div>
              <div>jsPDF - Client-side PDF generation</div>
              <div>Rust printpdf - Server-side PDF operations</div>
            </div>
            <p className="text-muted-foreground mt-2">
              Basic printing available now. Advanced PDF operations require additional dependencies.
            </p>
          </div>
        </section>

        {/* PDF Content Editor */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <File className="w-5 h-5" />
            PDF Content
          </h2>

          <div className="space-y-3">
            <div className="space-y-2">
              <label htmlFor="pdf-title" className="text-sm font-medium">
                Document Title
              </label>
              <Input
                id="pdf-title"
                value={pdfTitle}
                onChange={(e) => setPdfTitle(e.target.value)}
                placeholder="Enter document title"
                className="w-full"
              />
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
                Supports HTML tags: h1-h6, p, strong, em, ul, ol, li, etc.
              </p>
            </div>
          </div>
        </section>

        {/* Browser-Based PDF Operations */}
        <section className="rounded-lg border p-6 space-y-4">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Printer className="w-5 h-5" />
            Browser Print & PDF
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Use browser's built-in print functionality to generate PDFs or print documents
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">Generate PDF</h3>
                  <p className="text-xs text-muted-foreground">
                    Opens print dialog with your content (save as PDF)
                  </p>
                </div>
                <Button
                  onClick={handleGeneratePdfBrowser}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <FileText className="w-4 h-4 mr-2" />
                  Generate PDF
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">Print Current Page</h3>
                  <p className="text-xs text-muted-foreground">
                    Opens print dialog for this page
                  </p>
                </div>
                <Button
                  onClick={handlePrintCurrentPage}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <Printer className="w-4 h-4 mr-2" />
                  Print Page
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3">
                <div>
                  <h3 className="font-semibold mb-1">Preview PDF</h3>
                  <p className="text-xs text-muted-foreground">
                    Preview how the PDF will look
                  </p>
                </div>
                <Button
                  onClick={handlePreviewPdf}
                  variant="outline"
                  className="w-full"
                  disabled={isLoading}
                >
                  <FileText className="w-4 h-4 mr-2" />
                  Preview
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-3 border-yellow-500/30 bg-yellow-500/5">
                <div>
                  <h3 className="font-semibold mb-1">jsPDF Library</h3>
                  <p className="text-xs text-muted-foreground">
                    Requires installation of jsPDF
                  </p>
                </div>
                <Button
                  onClick={handleGeneratePdfJs}
                  variant="outline"
                  className="w-full border-yellow-500/50"
                  disabled={isLoading}
                >
                  <FileText className="w-4 h-4 mr-2" />
                  Generate with jsPDF
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* Advanced PDF Operations */}
        <section className="rounded-lg border p-6 space-y-4 border-purple-500/30 bg-purple-500/5">
          <h2 className="text-xl font-semibold flex items-center gap-2">
            <Files className="w-5 h-5" />
            Advanced PDF Operations
          </h2>

          <div className="space-y-3">
            <p className="text-sm text-muted-foreground">
              Advanced features require Rust backend implementation
            </p>

            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">Merge PDFs</h3>
                <p className="text-xs text-muted-foreground">
                  Combine multiple PDF files into one
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">Split PDF</h3>
                <p className="text-xs text-muted-foreground">
                  Divide PDF into separate files
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">Extract Text</h3>
                <p className="text-xs text-muted-foreground">
                  Get text content from PDF pages
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">Add Watermark</h3>
                <p className="text-xs text-muted-foreground">
                  Apply watermark to PDF pages
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">PDF Info</h3>
                <p className="text-xs text-muted-foreground">
                  Get metadata and properties
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>

              <div className="border rounded-lg p-4 space-y-2 opacity-50">
                <h3 className="font-semibold text-sm">Encrypt PDF</h3>
                <p className="text-xs text-muted-foreground">
                  Password protect PDF files
                </p>
                <Button variant="outline" className="w-full" disabled>
                  Coming Soon
                </Button>
              </div>
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
              <p className="text-muted-foreground">No output yet. Try generating or printing a PDF...</p>
            ) : (
              output.map((line, i) => (
                <div key={i} className="mb-1">
                  {line}
                </div>
              ))
            )}
          </div>
        </section>

        {/* Implementation Guide */}
        <section className="rounded-lg border border-blue-500/50 bg-blue-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Implementation Guide</h3>
          <div className="space-y-4 text-sm">
            <div className="space-y-2">
              <h4 className="font-semibold">Browser-Based (Available Now)</h4>
              <p className="text-muted-foreground">
                Use native browser APIs for basic PDF generation and printing
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Print current page or custom content</div>
                <div>window.print()</div>
                <div className="mt-2">// Print custom content in new window</div>
                <div>const printWin = window.open('', '', 'width=800')</div>
                <div>printWin.document.write(htmlContent)</div>
                <div>printWin.print()</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">jsPDF Library (Requires Installation)</h4>
              <p className="text-muted-foreground">
                Client-side PDF generation with more control
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div># Install jsPDF</div>
                <div>bun add jspdf</div>
                <div className="mt-2">// Generate PDF</div>
                <div>import jsPDF from 'jspdf'</div>
                <div>const doc = new jsPDF()</div>
                <div>doc.text('Hello world', 10, 10)</div>
                <div>doc.save('document.pdf')</div>
              </div>
            </div>

            <div className="space-y-2">
              <h4 className="font-semibold">Rust Backend (Future Implementation)</h4>
              <p className="text-muted-foreground">
                Server-side PDF operations with printpdf/lopdf crates
              </p>
              <div className="bg-muted/50 rounded-md p-3 font-mono text-xs space-y-1">
                <div>// Add to Cargo.toml</div>
                <div>printpdf = "0.7"</div>
                <div>lopdf = "0.32"</div>
                <div className="mt-2">// Tauri command</div>
                <div>#[tauri::command]</div>
                <div>async fn generate_pdf(content: String) -&gt; Result&lt;String&gt;</div>
              </div>
            </div>

            <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-md p-4">
              <h4 className="font-semibold mb-2 text-yellow-700 dark:text-yellow-400">
                Permissions & Considerations
              </h4>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2 text-xs">
                <li>Browser print may be blocked by popup blockers</li>
                <li>File system access requires Tauri dialog permissions</li>
                <li>Large PDFs may require streaming for performance</li>
                <li>PDF encryption requires careful key management</li>
                <li>Consider privacy when handling document content</li>
                <li>Test print layouts on different paper sizes</li>
                <li>Verify PDF/A compliance for archival documents</li>
              </ul>
            </div>
          </div>
        </section>

        {/* Platform Support */}
        <section className="rounded-lg border border-purple-500/50 bg-purple-500/5 p-6">
          <h3 className="text-lg font-semibold mb-3">Platform Support</h3>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b">
                  <th className="text-left py-2 px-4">Feature</th>
                  <th className="text-center py-2 px-4">Windows</th>
                  <th className="text-center py-2 px-4">macOS</th>
                  <th className="text-center py-2 px-4">Linux</th>
                  <th className="text-center py-2 px-4">Mobile</th>
                </tr>
              </thead>
              <tbody className="text-muted-foreground">
                <tr className="border-b">
                  <td className="py-2 px-4">Browser Print</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Generate PDF</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">System Print Dialog</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">✅</td>
                  <td className="text-center py-2 px-4">⚠️*</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">PDF Viewer</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Merge PDFs</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                </tr>
                <tr className="border-b">
                  <td className="py-2 px-4">Extract Text</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                </tr>
                <tr>
                  <td className="py-2 px-4">PDF Encryption</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                  <td className="text-center py-2 px-4">⏳</td>
                </tr>
              </tbody>
            </table>
            <div className="text-xs text-muted-foreground mt-2 space-y-1">
              <p>* Mobile uses share functionality for printing</p>
              <p>⏳ Planned feature, requires implementation</p>
            </div>
          </div>
        </section>
      </div>
    </ModulePageLayout>
  )
}
