use crate::models::llm_config::{LlmConfig, LlmProvider as LlmProviderEnum};
use academic_paper_interpreter::agents::{
    AnthropicProvider, OllamaProvider, OpenAiProvider, PaperAnalyzer,
};
use academic_paper_interpreter::client::PaperClient;
use shared::errors::{AppError, AppResult};

/// Enum to hold different analyzer types
pub enum AnalyzerType {
    OpenAi(PaperAnalyzer<OpenAiProvider>),
    Anthropic(PaperAnalyzer<AnthropicProvider>),
    Ollama(PaperAnalyzer<OllamaProvider>),
}

/// Create a PaperAnalyzer based on the LLM configuration
pub fn create_analyzer(config: &LlmConfig) -> AppResult<AnalyzerType> {
    match config.provider {
        LlmProviderEnum::OpenAi => {
            let provider = OpenAiProvider::from_env()
                .map_err(|e| AppError::LlmConfigError(format!("OpenAI provider error: {}", e)))?;
            Ok(AnalyzerType::OpenAi(PaperAnalyzer::new(provider)))
        }
        LlmProviderEnum::Anthropic => {
            let provider = AnthropicProvider::from_env()
                .map_err(|e| AppError::LlmConfigError(format!("Anthropic provider error: {}", e)))?;
            Ok(AnalyzerType::Anthropic(PaperAnalyzer::new(provider)))
        }
        LlmProviderEnum::Ollama => {
            let provider = OllamaProvider::from_env()
                .map_err(|e| AppError::LlmConfigError(format!("Ollama provider error: {}", e)))?;
            Ok(AnalyzerType::Ollama(PaperAnalyzer::new(provider)))
        }
    }
}

/// Create a PaperClient for searching and fetching papers
pub fn create_paper_client() -> PaperClient {
    PaperClient::new()
}
