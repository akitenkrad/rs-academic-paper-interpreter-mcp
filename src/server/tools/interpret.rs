use crate::models::analysis::AnalysisType;
use crate::models::request::{AnalyzePaperRequest, FetchPaperRequest, InterpretPaperRequest};
use crate::models::response::{AnalyzePaperResponse, FetchPaperResponse, InterpretPaperResponse};
use crate::server::handler::PaperInterpreterService;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use rmcp::Error as McpError;

impl PaperInterpreterService {
    #[tool(description = "Search, fetch, and analyze a paper in one operation")]
    pub async fn interpret_paper(
        &self,
        #[tool(aggr)] request: InterpretPaperRequest,
    ) -> Result<CallToolResult, McpError> {
        let query = &request.query;

        // Validate query has at least one identifier
        if !query.has_identifier() {
            return Err(McpError::invalid_params(
                "At least one query parameter (title, url, pdf_url, or arxiv_id) is required",
                None,
            ));
        }

        tracing::info!("Interpreting paper with query: {:?}", query);

        // Step 1: Fetch the paper
        let fetch_request = FetchPaperRequest {
            arxiv_id: query.arxiv_id.clone(),
            url: query.url.clone(),
            include_pdf_content: true,
        };

        let fetch_result = self.fetch_paper(fetch_request).await?;

        // Parse the fetch response to extract paper
        let fetch_content = fetch_result
            .content
            .first()
            .and_then(|c| c.as_text())
            .map(|t| t.text.as_str())
            .ok_or_else(|| McpError::internal_error("Failed to extract fetch result", None))?;

        let fetch_response: FetchPaperResponse = serde_json::from_str(fetch_content).map_err(
            |e| McpError::internal_error(format!("Failed to parse fetch response: {}", e), None),
        )?;

        // Step 2: Analyze the paper
        let analyze_request = AnalyzePaperRequest {
            paper: fetch_response.paper.clone(),
            llm_config: request.llm_config.clone(),
            analysis_type: AnalysisType::Summary,
        };

        let analyze_result = self.analyze_paper(analyze_request).await?;

        // Parse analysis response
        let analyze_content = analyze_result
            .content
            .first()
            .and_then(|c| c.as_text())
            .map(|t| t.text.as_str())
            .ok_or_else(|| McpError::internal_error("Failed to extract analysis result", None))?;

        let analyze_response: AnalyzePaperResponse =
            serde_json::from_str(analyze_content).map_err(|e| {
                McpError::internal_error(format!("Failed to parse analysis response: {}", e), None)
            })?;

        // Combine results
        let response = InterpretPaperResponse {
            paper: fetch_response.paper,
            analysis: analyze_response.analysis,
        };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}
