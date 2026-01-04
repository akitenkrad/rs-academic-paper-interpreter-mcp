use crate::models::llm_config::{LlmConfig, LlmProvider};
use shared::errors::{AppError, AppResult};
use std::env;

/// Resolves LLM configuration from environment and parameters
pub struct LlmConfigResolver {
    default_config: LlmConfig,
}

impl LlmConfigResolver {
    pub fn new() -> Self {
        Self {
            default_config: LlmConfig::from_env(),
        }
    }

    /// Resolve configuration with optional overrides
    pub fn resolve(&self, override_config: Option<&LlmConfig>) -> LlmConfig {
        self.default_config.merge_with(override_config)
    }

    /// Validate that required API keys are present
    pub fn validate_api_key(&self, config: &LlmConfig) -> AppResult<()> {
        match config.provider {
            LlmProvider::OpenAi => {
                if env::var("OPENAI_API_KEY").is_err() {
                    return Err(AppError::LlmConfigError(
                        "OPENAI_API_KEY environment variable not set".to_string(),
                    ));
                }
            }
            LlmProvider::Anthropic => {
                if env::var("ANTHROPIC_API_KEY").is_err() {
                    return Err(AppError::LlmConfigError(
                        "ANTHROPIC_API_KEY environment variable not set".to_string(),
                    ));
                }
            }
            LlmProvider::Ollama => {
                // Ollama doesn't require API key, but check server URL
                let base_url = env::var("OLLAMA_BASE_URL")
                    .unwrap_or_else(|_| "http://localhost:11434".to_string());
                tracing::debug!("Using Ollama at {}", base_url);
            }
        }
        Ok(())
    }
}

impl Default for LlmConfigResolver {
    fn default() -> Self {
        Self::new()
    }
}
