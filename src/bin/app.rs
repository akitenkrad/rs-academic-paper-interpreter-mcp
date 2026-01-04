use app::server::transport::run_stdio_server;
use clap::Parser;
use shared::errors::AppResult;
use shared::logger::init_logger;

#[derive(Parser, Debug)]
#[command(name = "academic-paper-interpreter-mcp")]
#[command(about = "MCP server for academic paper interpretation")]
#[command(version)]
struct Args {
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
    tracing::info!("Transport: stdio");

    run_stdio_server().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let args = Args::parse_from(["app"]);
        assert_eq!(args.log_level, "info");

        let args = Args::parse_from(["app", "--log-level", "debug"]);
        assert_eq!(args.log_level, "debug");
    }
}
