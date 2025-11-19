# Printing & PDF / Document Handling Module Implementation

## Overview

The Printing & PDF Module provides comprehensive document handling capabilities, including PDF generation, viewing, annotation, and system printing. This module enables users to create documents from content, preview PDFs, trigger native print dialogs, and perform various document operations.

## Current Implementation Status

⏳ **Status**: In Progress

## Plugin Setup

### Dependencies

**Frontend Libraries**
```bash
# PDF generation
bun add jspdf

# PDF viewing
bun add react-pdf pdfjs-dist

# PDF annotation (optional)
bun add @react-pdf-viewer/core @react-pdf-viewer/default-layout
```

### Cargo Dependencies

```toml
[dependencies]
# For PDF generation in Rust (optional)
printpdf = "0.7"
lopdf = "0.32"

# For system print dialog
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.58", features = ["Win32_UI_Shell"] }

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
objc = "0.2"
```

### Plugin Registration

```rust
// Commands registered in src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_pdf,
            open_print_dialog,
            save_pdf,
            get_pdf_info,
            merge_pdfs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Permissions Configuration

### Tauri Capabilities

Add to `src-tauri/capabilities/default.json`:

```json
{
  "permissions": [
    "core:default",
    "dialog:allow-save",
    "dialog:allow-open",
    "fs:allow-write-file",
    "fs:allow-read-file",
    "shell:allow-open"
  ]
}
```

### Platform-Specific Permissions

**macOS**: No additional permissions required for print dialog

**Windows**: System print dialog accessible through Windows API

**Linux**: Requires `gtk-3.0` for native print dialog

## Core Features

- [ ] Generate PDF from HTML content
- [ ] Generate PDF from plain text
- [ ] Open system print dialog
- [ ] Preview PDF before printing
- [ ] Save PDF to file system
- [ ] Load and display PDF files
- [ ] Get PDF metadata (pages, size, author, etc.)
- [ ] Merge multiple PDFs
- [ ] Split PDF into multiple files
- [ ] Add annotations to PDF
- [ ] Extract text from PDF
- [ ] Convert images to PDF
- [ ] Add watermarks to PDF
- [ ] Encrypt/decrypt PDF

## Data Structures

### TypeScript Interfaces

```typescript
// PDF generation options
interface PdfGenerationOptions {
  title?: string;
  author?: string;
  subject?: string;
  keywords?: string[];
  format?: 'A4' | 'Letter' | 'Legal';
  orientation?: 'portrait' | 'landscape';
  margins?: {
    top: number;
    right: number;
    bottom: number;
    left: number;
  };
}

// PDF metadata
interface PdfMetadata {
  title: string;
  author: string;
  subject: string;
  creator: string;
  producer: string;
  creationDate: string;
  modificationDate: string;
  pageCount: number;
  fileSize: number;
}

// Print options
interface PrintOptions {
  copies?: number;
  color?: boolean;
  duplex?: 'simplex' | 'duplex-long-edge' | 'duplex-short-edge';
  pageRanges?: string; // e.g., "1-5,8,11-13"
}

// PDF annotation
interface PdfAnnotation {
  type: 'text' | 'highlight' | 'underline' | 'strikeout';
  page: number;
  x: number;
  y: number;
  width: number;
  height: number;
  content?: string;
  color?: string;
}
```

## Rust Backend

### Commands

```rust
#[tauri::command]
async fn generate_pdf(
    content: String,
    options: PdfGenerationOptions,
    output_path: String,
) -> Result<String, String> {
    // Generate PDF from content and save to file
}

#[tauri::command]
async fn open_print_dialog(file_path: String) -> Result<(), String> {
    // Open native system print dialog
}

#[tauri::command]
async fn save_pdf(
    content: Vec<u8>,
    file_path: String,
) -> Result<(), String> {
    // Save PDF bytes to file system
}

#[tauri::command]
async fn get_pdf_info(file_path: String) -> Result<PdfMetadata, String> {
    // Extract PDF metadata
}

#[tauri::command]
async fn merge_pdfs(
    input_paths: Vec<String>,
    output_path: String,
) -> Result<(), String> {
    // Merge multiple PDFs into one
}

#[tauri::command]
async fn split_pdf(
    input_path: String,
    page_ranges: Vec<(u32, u32)>,
    output_dir: String,
) -> Result<Vec<String>, String> {
    // Split PDF into multiple files
}

#[tauri::command]
async fn extract_text(
    file_path: String,
    page_number: Option<u32>,
) -> Result<String, String> {
    // Extract text from PDF page(s)
}
```

### Windows Implementation

```rust
#[cfg(target_os = "windows")]
async fn open_print_dialog(file_path: String) -> Result<(), String> {
    use windows::Win32::UI::Shell::ShellExecuteW;
    use std::os::windows::ffi::OsStrExt;

    // Open print dialog using Windows Shell
    // ShellExecuteW with "print" verb
}
```

### macOS Implementation

```rust
#[cfg(target_os = "macos")]
async fn open_print_dialog(file_path: String) -> Result<(), String> {
    use std::process::Command;

    // Use 'lpr' command or NSPrintPanel
    Command::new("lpr")
        .arg(&file_path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

### Linux Implementation

```rust
#[cfg(target_os = "linux")]
async fn open_print_dialog(file_path: String) -> Result<(), String> {
    use std::process::Command;

    // Use 'lp' or 'lpr' command
    Command::new("lp")
        .arg(&file_path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

## Frontend Implementation

### React Hook

```typescript
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';

export function usePdfOperations() {
  const generatePdf = async (
    content: string,
    options: PdfGenerationOptions
  ) => {
    try {
      const savePath = await save({
        defaultPath: 'document.pdf',
        filters: [{ name: 'PDF', extensions: ['pdf'] }],
      });

      if (savePath) {
        return await invoke<string>('generate_pdf', {
          content,
          options,
          outputPath: savePath,
        });
      }
    } catch (error) {
      console.error('PDF generation failed:', error);
      throw error;
    }
  };

  const openPrintDialog = async (filePath: string) => {
    try {
      await invoke('open_print_dialog', { filePath });
    } catch (error) {
      console.error('Print dialog failed:', error);
      throw error;
    }
  };

  const getPdfInfo = async (filePath: string) => {
    try {
      return await invoke<PdfMetadata>('get_pdf_info', { filePath });
    } catch (error) {
      console.error('Get PDF info failed:', error);
      throw error;
    }
  };

  const mergePdfs = async (inputPaths: string[], outputPath: string) => {
    try {
      await invoke('merge_pdfs', { inputPaths, outputPath });
    } catch (error) {
      console.error('PDF merge failed:', error);
      throw error;
    }
  };

  return {
    generatePdf,
    openPrintDialog,
    getPdfInfo,
    mergePdfs,
  };
}
```

### Component Usage

```tsx
import { usePdfOperations } from '@/hooks/usePdfOperations';
import { Button } from '@/components/ui/button';

function PdfDemo() {
  const { generatePdf, openPrintDialog } = usePdfOperations();

  const handleGeneratePdf = async () => {
    await generatePdf(
      '<h1>Hello World</h1><p>This is a test document.</p>',
      {
        title: 'Test Document',
        format: 'A4',
        orientation: 'portrait',
      }
    );
  };

  return (
    <div>
      <Button onClick={handleGeneratePdf}>Generate PDF</Button>
      <Button onClick={() => openPrintDialog('/path/to/file.pdf')}>
        Print Document
      </Button>
    </div>
  );
}
```

## Browser-Based PDF Generation

### Using jsPDF

```typescript
import jsPDF from 'jspdf';

export function generatePdfClient(content: string, options: PdfGenerationOptions) {
  const doc = new jsPDF({
    orientation: options.orientation || 'portrait',
    unit: 'mm',
    format: options.format || 'A4',
  });

  // Add content
  doc.setProperties({
    title: options.title || 'Document',
    author: options.author || '',
    subject: options.subject || '',
  });

  // Parse HTML or add text
  doc.html(content, {
    callback: (doc) => {
      doc.save('document.pdf');
    },
    x: options.margins?.left || 10,
    y: options.margins?.top || 10,
  });

  return doc;
}
```

### Using window.print()

```typescript
export function printHtml(content: string) {
  const printWindow = window.open('', '', 'width=800,height=600');

  if (printWindow) {
    printWindow.document.write(content);
    printWindow.document.close();
    printWindow.focus();
    printWindow.print();
    printWindow.close();
  }
}
```

## UI Components

- **PDF Generation Section**: Input for content, options selector, generate button
- **Print Section**: File picker, print dialog trigger
- **PDF Viewer**: Display PDF with navigation controls
- **PDF Info Section**: Display metadata and properties
- **PDF Operations**: Merge, split, extract text controls
- **Output Log**: Real-time feedback on PDF operations

## Testing Checklist

### PDF Generation
- [ ] Generate PDF from HTML content
- [ ] Generate PDF from plain text
- [ ] Test different page formats (A4, Letter, Legal)
- [ ] Test portrait and landscape orientations
- [ ] Verify PDF metadata is set correctly
- [ ] Test custom margins

### Printing
- [ ] Open system print dialog on Windows
- [ ] Open system print dialog on macOS
- [ ] Open system print dialog on Linux
- [ ] Print multi-page documents
- [ ] Test print preview functionality

### PDF Operations
- [ ] Load and display PDF files
- [ ] Extract PDF metadata
- [ ] Merge multiple PDFs successfully
- [ ] Split PDF into separate files
- [ ] Extract text from PDF pages

### Cross-Platform
- [ ] Test on Windows 10/11
- [ ] Test on macOS (latest version)
- [ ] Test on Linux (Ubuntu/Fedora)
- [ ] Verify file dialogs work correctly
- [ ] Check file permissions handling

## Troubleshooting

### Common Issues

**PDF Generation Fails**
- Check write permissions to output directory
- Verify content is valid HTML/text
- Ensure sufficient disk space

**Print Dialog Not Opening**
- Verify system print services are running
- Check file path is valid and accessible
- Ensure PDF viewer is installed (for some systems)

**PDF Merge Fails**
- Verify all input PDFs are valid
- Check PDFs are not password-protected
- Ensure sufficient memory for large files

**Text Extraction Returns Empty**
- Some PDFs may be image-based (need OCR)
- Check PDF is not encrypted
- Verify page number is valid

## Resources

### Libraries
- [jsPDF Documentation](https://github.com/parallax/jsPDF)
- [react-pdf Documentation](https://github.com/wojtekmaj/react-pdf)
- [printpdf Rust Crate](https://crates.io/crates/printpdf)
- [lopdf Rust Crate](https://crates.io/crates/lopdf)

### APIs
- [Windows Print API](https://docs.microsoft.com/en-us/windows/win32/printdocs/printing-and-print-spooler)
- [macOS NSPrintPanel](https://developer.apple.com/documentation/appkit/nsprintpanel)
- [PDF Reference](https://www.adobe.com/content/dam/acom/en/devnet/pdf/pdfs/PDF32000_2008.pdf)

## Platform Support

| Feature | Windows | macOS | Linux | Android | iOS |
|---------|---------|-------|-------|---------|-----|
| Generate PDF | ✅ | ✅ | ✅ | ✅ | ✅ |
| Print Dialog | ✅ | ✅ | ✅ | ⚠️ | ⚠️ |
| PDF Viewer | ✅ | ✅ | ✅ | ✅ | ✅ |
| Merge PDFs | ✅ | ✅ | ✅ | ✅ | ✅ |
| Split PDF | ✅ | ✅ | ✅ | ✅ | ✅ |
| Extract Text | ✅ | ✅ | ✅ | ✅ | ✅ |
| Annotations | ✅ | ✅ | ✅ | ✅ | ✅ |
| Encryption | ✅ | ✅ | ✅ | ✅ | ✅ |

**Legend:**
- ✅ Fully Supported
- ⚠️ Limited Support (mobile print may use share functionality)
- ❌ Not Supported

## Implementation Status

### Phase 1: Core Setup
- [ ] Add frontend dependencies (jsPDF, react-pdf)
- [ ] Add Rust dependencies (printpdf, lopdf)
- [ ] Register Tauri commands
- [ ] Configure file system permissions

### Phase 2: Basic PDF Operations
- [ ] Implement PDF generation from HTML
- [ ] Implement PDF generation from text
- [ ] Implement save PDF to file system
- [ ] Add basic error handling

### Phase 3: Printing
- [ ] Implement system print dialog (Windows)
- [ ] Implement system print dialog (macOS)
- [ ] Implement system print dialog (Linux)
- [ ] Add print options support

### Phase 4: Advanced Features
- [ ] Implement PDF viewer component
- [ ] Implement PDF metadata extraction
- [ ] Implement PDF merge functionality
- [ ] Implement PDF split functionality
- [ ] Implement text extraction

### Phase 5: Frontend Integration
- [ ] Create React hooks for PDF operations
- [ ] Build UI demo page
- [ ] Add PDF preview functionality
- [ ] Implement output logging

### Phase 6: Testing & Polish
- [ ] Test on all desktop platforms
- [ ] Test on mobile platforms
- [ ] Add user documentation
- [ ] Performance optimization
- [ ] Error handling improvements
