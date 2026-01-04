use crate::llm::create_paper_client;
use crate::models::paper::PaperSummary;
use crate::models::request::SearchPapersRequest;
use crate::models::response::SearchPapersResponse;
use crate::server::handler::PaperInterpreterService;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use rmcp::Error as McpError;

impl PaperInterpreterService {
    #[tool(description = "Search for academic papers by keywords, author, or category")]
    pub async fn search_papers(
        &self,
        #[tool(aggr)] request: SearchPapersRequest,
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Searching papers with query: {}", request.query);

        let client = create_paper_client();

        // Build search params
        let mut params = academic_paper_interpreter::client::SearchParams::default();
        params.query = Some(request.query.clone());

        // Execute search
        let search_result = client
            .search(params)
            .await
            .map_err(|e| McpError::internal_error(format!("Search failed: {}", e), None))?;

        // Convert to response format - SearchResult has a papers field
        let summaries: Vec<PaperSummary> = search_result
            .papers
            .into_iter()
            .take(request.max_results as usize)
            .map(|p| PaperSummary {
                title: p.title,
                authors: p.authors.into_iter().map(|a| a.name).collect(),
                abstract_text: p.abstract_text,
                arxiv_id: if p.arxiv_id.is_empty() {
                    None
                } else {
                    Some(p.arxiv_id)
                },
                published_date: Some(p.published_date.to_rfc3339()),
                pdf_url: if p.url.is_empty() { None } else { Some(p.url) },
            })
            .collect();

        let response = SearchPapersResponse {
            total_count: summaries.len() as u32,
            papers: summaries,
        };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}
