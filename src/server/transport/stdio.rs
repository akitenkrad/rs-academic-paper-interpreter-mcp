use crate::server::PaperInterpreterService;
use rmcp::transport::stdio;
use rmcp::ServiceExt;
use shared::errors::{AppError, AppResult};

/// Run the MCP server with stdio transport
pub async fn run_stdio_server() -> AppResult<()> {
    tracing::info!("Starting MCP server with stdio transport");

    let transport = stdio();
    let service = PaperInterpreterService::new();

    let server = service
        .serve(transport)
        .await
        .map_err(|e| AppError::InternalAppError(format!("Failed to start stdio server: {}", e)))?;

    tracing::info!("MCP server running on stdio");

    server
        .waiting()
        .await
        .map_err(|e| AppError::InternalAppError(format!("Server error: {}", e)))?;

    Ok(())
}
