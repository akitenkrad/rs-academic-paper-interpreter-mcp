use crate::server::PaperInterpreterService;
use rmcp::transport::SseServer;
use shared::errors::{AppError, AppResult};
use std::net::SocketAddr;

/// Run the MCP server with SSE transport
pub async fn run_sse_server(port: u16) -> AppResult<()> {
    tracing::info!("Starting MCP server with SSE transport on port {}", port);

    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .map_err(|e| AppError::InternalAppError(format!("Invalid address: {}", e)))?;

    let server = SseServer::serve(addr)
        .await
        .map_err(|e| AppError::InternalAppError(format!("Failed to start SSE server: {}", e)))?;

    tracing::info!("MCP server listening on http://{}", addr);
    tracing::info!("  SSE endpoint: http://{}/sse", addr);
    tracing::info!("  Message endpoint: http://{}/message", addr);

    let ct = server.with_service(PaperInterpreterService::new);

    // Wait for cancellation (Ctrl+C or service stop)
    tokio::signal::ctrl_c()
        .await
        .map_err(|e| AppError::InternalAppError(format!("Signal handler error: {}", e)))?;

    tracing::info!("Shutting down SSE server...");
    ct.cancel();

    Ok(())
}
