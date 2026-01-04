use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents an academic paper with metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Paper {
    #[schemars(description = "Paper title")]
    pub title: String,

    #[schemars(description = "List of author names")]
    pub authors: Vec<String>,

    #[schemars(description = "Paper abstract")]
    #[serde(rename = "abstract")]
    pub abstract_text: String,

    #[schemars(description = "arXiv identifier (e.g., 2301.00001)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arxiv_id: Option<String>,

    #[schemars(description = "Semantic Scholar paper ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ss_id: Option<String>,

    #[schemars(description = "arXiv categories (e.g., cs.CL, cs.AI)")]
    #[serde(default)]
    pub categories: Vec<String>,

    #[schemars(description = "Publication date in ISO 8601 format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,

    #[schemars(description = "URL to PDF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,

    #[schemars(description = "Full paper content (if fetched)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// Represents a paper search result with limited metadata
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PaperSummary {
    pub title: String,
    pub authors: Vec<String>,
    #[serde(rename = "abstract")]
    pub abstract_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arxiv_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
}
