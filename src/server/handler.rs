use crate::llm::LlmConfigResolver;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::service::{Peer, RoleServer};
use rmcp::tool;
use rmcp::ServerHandler;
use std::sync::Arc;

/// Main MCP service handler
#[derive(Clone)]
pub struct PaperInterpreterService {
    llm_config_resolver: Arc<LlmConfigResolver>,
    peer: Option<Peer<RoleServer>>,
}

#[tool(tool_box)]
impl PaperInterpreterService {
    pub fn new() -> Self {
        Self {
            llm_config_resolver: Arc::new(LlmConfigResolver::new()),
            peer: None,
        }
    }

    pub fn llm_config_resolver(&self) -> &LlmConfigResolver {
        &self.llm_config_resolver
    }
}

impl Default for PaperInterpreterService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHandler for PaperInterpreterService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "An MCP server for searching, fetching, and analyzing academic papers. \
                 Use interpret_paper for end-to-end analysis, or use search_papers, \
                 fetch_paper, and analyze_paper for granular control."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }

    fn get_peer(&self) -> Option<Peer<RoleServer>> {
        self.peer.clone()
    }

    fn set_peer(&mut self, peer: Peer<RoleServer>) {
        self.peer = Some(peer);
    }

    rmcp::tool_box!(@derive tool_box);
}
