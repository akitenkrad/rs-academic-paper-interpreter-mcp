use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

/// Supported LLM providers
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    #[default]
    OpenAi,
    Anthropic,
    Ollama,
}

/// LLM configuration with environment variable fallbacks
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct LlmConfig {
    #[schemars(description = "LLM provider: openai, anthropic, or ollama")]
    #[serde(default)]
    pub provider: LlmProvider,

    #[schemars(description = "Model name to use (provider-specific)")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl LlmConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        let provider = match env::var("LLM_PROVIDER")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "anthropic" => LlmProvider::Anthropic,
            "ollama" => LlmProvider::Ollama,
            _ => LlmProvider::OpenAi,
        };

        Self {
            provider,
            model: None,
        }
    }

    /// Merge with parameter overrides (parameters take precedence)
    pub fn merge_with(&self, override_config: Option<&LlmConfig>) -> Self {
        match override_config {
            Some(cfg) => Self {
                provider: cfg.provider.clone(),
                model: cfg.model.clone().or_else(|| self.model.clone()),
            },
            None => self.clone(),
        }
    }

    /// Get the effective model name for the provider
    pub fn effective_model(&self) -> String {
        if let Some(ref model) = self.model {
            return model.clone();
        }

        match self.provider {
            LlmProvider::OpenAi => {
                env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5.2-2025-12-11".to_string())
            }
            LlmProvider::Anthropic => env::var("ANTHROPIC_MODEL")
                .unwrap_or_else(|_| "claude-sonnet-4-20250514".to_string()),
            LlmProvider::Ollama => {
                env::var("OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string())
            }
        }
    }
}
