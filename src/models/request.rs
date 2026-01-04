use super::{analysis::AnalysisType, llm_config::LlmConfig, paper::Paper};
use schemars::JsonSchema;
use serde::Deserialize;

/// Request for interpret_paper tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct InterpretPaperRequest {
    #[schemars(description = "Query to identify the paper")]
    pub query: PaperQuery,

    #[schemars(description = "LLM configuration (optional, uses env defaults)")]
    #[serde(default)]
    pub llm_config: Option<LlmConfig>,
}

/// Query parameters to identify a paper
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PaperQuery {
    #[schemars(description = "Paper title for search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[schemars(description = "Paper URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[schemars(description = "PDF URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,

    #[schemars(description = "arXiv ID (e.g., 2301.00001)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arxiv_id: Option<String>,
}

impl PaperQuery {
    /// Check if at least one identifier is provided
    pub fn has_identifier(&self) -> bool {
        self.title.is_some()
            || self.url.is_some()
            || self.pdf_url.is_some()
            || self.arxiv_id.is_some()
    }
}

/// Request for search_papers tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SearchPapersRequest {
    #[schemars(description = "Search keywords")]
    pub query: String,

    #[schemars(description = "Filter by author name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[schemars(description = "arXiv category filter (e.g., cs.CL)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    #[schemars(description = "Maximum number of results")]
    #[serde(default = "default_max_results")]
    pub max_results: u32,
}

fn default_max_results() -> u32 {
    10
}

/// Request for fetch_paper tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FetchPaperRequest {
    #[schemars(description = "arXiv ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arxiv_id: Option<String>,

    #[schemars(description = "Paper URL (if no arxiv_id)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[schemars(description = "Include PDF content in response")]
    #[serde(default = "default_include_pdf")]
    pub include_pdf_content: bool,
}

fn default_include_pdf() -> bool {
    true
}

impl FetchPaperRequest {
    /// Check if at least one identifier is provided
    pub fn has_identifier(&self) -> bool {
        self.arxiv_id.is_some() || self.url.is_some()
    }
}

/// Request for analyze_paper tool
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AnalyzePaperRequest {
    #[schemars(description = "Paper data from fetch_paper")]
    pub paper: Paper,

    #[schemars(description = "LLM configuration")]
    #[serde(default)]
    pub llm_config: Option<LlmConfig>,

    #[schemars(description = "Type of analysis")]
    #[serde(default)]
    pub analysis_type: AnalysisType,
}
