use crate::llm::{create_analyzer, AnalyzerType};
use crate::models::analysis::PaperAnalysis;
use crate::models::request::AnalyzePaperRequest;
use crate::models::response::AnalyzePaperResponse;
use crate::server::handler::PaperInterpreterService;
use academic_paper_interpreter::agents::AnalysisAgent;
use academic_paper_interpreter::models::{AcademicPaper, Author};
use chrono::Local;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use rmcp::Error as McpError;

impl PaperInterpreterService {
    #[tool(description = "Analyze a paper using an LLM to generate summary and insights")]
    pub async fn analyze_paper(
        &self,
        #[tool(aggr)] request: AnalyzePaperRequest,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Analyzing paper: {}", request.paper.title);

        // Resolve LLM configuration
        let config = self
            .llm_config_resolver()
            .resolve(request.llm_config.as_ref());

        // Validate API key availability
        self.llm_config_resolver()
            .validate_api_key(&config)
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        // Create analyzer with the configured provider
        let analyzer = create_analyzer(&config).map_err(|e| {
            McpError::internal_error(format!("Failed to create analyzer: {}", e), None)
        })?;

        // Convert our Paper to the library's AcademicPaper using the builder or new
        let mut academic_paper = AcademicPaper::default();
        academic_paper.title = request.paper.title.clone();
        academic_paper.authors = request
            .paper
            .authors
            .iter()
            .map(|name| Author {
                name: name.clone(),
                ss_id: String::new(),
                h_index: 0,
                affiliations: vec![],
                paper_count: 0,
                citation_count: 0,
            })
            .collect();
        academic_paper.abstract_text = request.paper.abstract_text.clone();
        academic_paper.arxiv_id = request.paper.arxiv_id.clone().unwrap_or_default();
        academic_paper.ss_id = request.paper.ss_id.clone().unwrap_or_default();
        academic_paper.url = request.paper.pdf_url.clone().unwrap_or_default();
        academic_paper.text = request.paper.content.clone().unwrap_or_default();
        academic_paper.published_date = Local::now();

        // Execute analysis based on provider type
        let lib_analysis = match analyzer {
            AnalyzerType::OpenAi(a) => a.analyze(&academic_paper).await.map_err(|e| {
                McpError::internal_error(format!("Analysis failed: {}", e), None)
            })?,
            AnalyzerType::Anthropic(a) => a.analyze(&academic_paper).await.map_err(|e| {
                McpError::internal_error(format!("Analysis failed: {}", e), None)
            })?,
            AnalyzerType::Ollama(a) => a.analyze(&academic_paper).await.map_err(|e| {
                McpError::internal_error(format!("Analysis failed: {}", e), None)
            })?,
        };

        // Convert to our response format
        let analysis = PaperAnalysis {
            summary: lib_analysis.summary,
            key_contributions: lib_analysis.key_contributions,
            methodology: if lib_analysis.methodology.is_empty() {
                None
            } else {
                Some(lib_analysis.methodology)
            },
            limitations: if lib_analysis.advantages_limitations_and_future_work.is_empty() {
                vec![]
            } else {
                vec![lib_analysis.advantages_limitations_and_future_work]
            },
            related_work: vec![],
        };

        let response = AnalyzePaperResponse { analysis };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}
