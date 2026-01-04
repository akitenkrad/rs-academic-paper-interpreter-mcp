use app::server::transport::{run_sse_server, run_stdio_server};
use clap::{Parser, ValueEnum};
use shared::errors::AppResult;
use shared::logger::init_logger;

#[derive(Debug, Clone, ValueEnum)]
enum Transport {
    Stdio,
    Sse,
}

#[derive(Parser, Debug)]
#[command(name = "academic-paper-interpreter-mcp")]
#[command(about = "MCP server for academic paper interpretation")]
#[command(version)]
struct Args {
    /// Transport to use
    #[arg(short, long, value_enum, default_value_t = Transport::Stdio)]
    transport: Transport,

    /// Port for SSE transport (ignored for stdio)
    #[arg(short, long, default_value_t = 18080)]
    port: u16,

    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();

    // Initialize logging
    init_logger(&args.log_level)?;

    tracing::info!("Academic Paper Interpreter MCP Server");
    tracing::info!("Transport: {:?}", args.transport);

    match args.transport {
        Transport::Stdio => run_stdio_server().await,
        Transport::Sse => run_sse_server(args.port).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = Args::parse_from(["app", "--transport", "stdio"]);
        assert!(matches!(args.transport, Transport::Stdio));

        let args = Args::parse_from(["app", "--transport", "sse", "--port", "9000"]);
        assert!(matches!(args.transport, Transport::Sse));
        assert_eq!(args.port, 9000);
    }
}
