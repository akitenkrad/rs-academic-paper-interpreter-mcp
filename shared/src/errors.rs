use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    InternalAppError(String),

    // from anyhow
    #[error("Error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    // from tracing
    #[error("Tracing Error: {0}")]
    TracingTryInitError(#[from] tracing_subscriber::util::TryInitError),

    // MCP-specific errors
    #[error("Paper not found: {0}")]
    PaperNotFound(String),

    #[error("Invalid arXiv ID: {0}")]
    InvalidArxivId(String),

    #[error("PDF fetch failed: {0}")]
    PdfFetchFailed(String),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("LLM configuration error: {0}")]
    LlmConfigError(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl AppError {
    /// Returns the MCP error code for this error type
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::PaperNotFound(_) => "PAPER_NOT_FOUND",
            AppError::InvalidArxivId(_) => "INVALID_ARXIV_ID",
            AppError::PdfFetchFailed(_) => "PDF_FETCH_FAILED",
            AppError::LlmError(_) => "LLM_ERROR",
            AppError::LlmConfigError(_) => "LLM_CONFIG_ERROR",
            AppError::RateLimitExceeded(_) => "RATE_LIMIT",
            AppError::NetworkError(_) => "NETWORK_ERROR",
            AppError::InvalidRequest(_) => "INVALID_REQUEST",
            _ => "INTERNAL_ERROR",
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
