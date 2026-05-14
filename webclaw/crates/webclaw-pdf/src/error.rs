/// PDF extraction errors. Kept simple -- no OCR, no complex recovery.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("PDF extraction failed: {0}")]
    ExtractionFailed(String),

    #[error("invalid PDF: {0}")]
    InvalidPdf(String),

    #[error("empty PDF: no text content found")]
    EmptyPdf,
}
