use super::{
    analysis::PaperAnalysis,
    paper::{Paper, PaperSummary},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Response for interpret_paper tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InterpretPaperResponse {
    pub paper: Paper,
    pub analysis: PaperAnalysis,
}

/// Response for search_papers tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchPapersResponse {
    pub papers: Vec<PaperSummary>,
    pub total_count: u32,
}

/// Response for fetch_paper tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FetchPaperResponse {
    pub paper: Paper,
}

/// Response for analyze_paper tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzePaperResponse {
    pub analysis: PaperAnalysis,
}

/// MCP error response structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct McpErrorResponse {
    pub error: McpErrorDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct McpErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}
