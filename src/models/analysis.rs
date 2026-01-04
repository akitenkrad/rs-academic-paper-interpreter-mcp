use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Analysis result from LLM processing
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PaperAnalysis {
    #[schemars(description = "Concise summary of the paper")]
    pub summary: String,

    #[schemars(description = "Key contributions of the paper")]
    pub key_contributions: Vec<String>,

    #[schemars(description = "Description of the methodology used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub methodology: Option<String>,

    #[schemars(description = "Identified limitations")]
    #[serde(default)]
    pub limitations: Vec<String>,

    #[schemars(description = "Related work references")]
    #[serde(default)]
    pub related_work: Vec<String>,
}

/// Type of analysis to perform
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisType {
    #[default]
    Summary,
    Detailed,
    Comparison,
}
