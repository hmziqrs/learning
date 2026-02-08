//! Error types for import/export operations

/// Detailed error types for import operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportErrorKind {
    /// I/O error (file not found, permission denied, etc.)
    Io,
    /// Failed to parse/deserialize the input
    Deserialization,
    /// Input format is not recognized or is invalid
    InvalidFormat,
    /// Data validation failed
    Validation,
    /// Postman-specific import error
    PostmanFormat,
    /// OpenAPI-specific import error
    OpenApiFormat,
}

/// Error that can occur during import operations
#[derive(Debug, Clone)]
pub struct ImportError {
    kind: ImportErrorKind,
    message: String,
}

impl ImportError {
    pub fn new(kind: ImportErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> &ImportErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for ImportError {}

/// Detailed error types for export operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportErrorKind {
    /// I/O error (failed to create file, disk full, etc.)
    Io,
    /// Failed to serialize data
    Serialization,
    /// Failed to create zip archive
    ZipError,
}

/// Error that can occur during export operations
#[derive(Debug, Clone)]
pub struct ExportError {
    kind: ExportErrorKind,
    message: String,
}

impl ExportError {
    pub fn new(kind: ExportErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }

    pub fn kind(&self) -> &ExportErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for ExportError {}

/// Convert from std::io::Error to ImportError
impl From<std::io::Error> for ImportError {
    fn from(err: std::io::Error) -> Self {
        ImportError::new(ImportErrorKind::Io, &err.to_string())
    }
}

/// Convert from std::io::Error to ExportError
impl From<std::io::Error> for ExportError {
    fn from(err: std::io::Error) -> Self {
        ExportError::new(ExportErrorKind::Io, &err.to_string())
    }
}

/// Convert from serde_json::Error to ImportError
impl From<serde_json::Error> for ImportError {
    fn from(err: serde_json::Error) -> Self {
        ImportError::new(ImportErrorKind::Deserialization, &err.to_string())
    }
}

/// Convert from serde_json::Error to ExportError
impl From<serde_json::Error> for ExportError {
    fn from(err: serde_json::Error) -> Self {
        ExportError::new(ExportErrorKind::Serialization, &err.to_string())
    }
}

/// Convert from zip::result::ZipError to ImportError
impl From<zip::result::ZipError> for ImportError {
    fn from(err: zip::result::ZipError) -> Self {
        ImportError::new(ImportErrorKind::InvalidFormat, &err.to_string())
    }
}

/// Convert from zip::result::ZipError to ExportError
impl From<zip::result::ZipError> for ExportError {
    fn from(err: zip::result::ZipError) -> Self {
        ExportError::new(ExportErrorKind::ZipError, &err.to_string())
    }
}

/// Result type for import operations
pub type ImportResult<T> = Result<T, ImportError>;

/// Result type for export operations
pub type ExportResult<T> = Result<T, ExportError>;
