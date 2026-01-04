use crate::llm::create_paper_client;
use crate::models::paper::Paper;
use crate::models::request::FetchPaperRequest;
use crate::models::response::FetchPaperResponse;
use crate::server::handler::PaperInterpreterService;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use rmcp::Error as McpError;

impl PaperInterpreterService {
    #[tool(description = "Fetch a paper's metadata and optionally its PDF content")]
    pub async fn fetch_paper(
        &self,
        #[tool(aggr)] request: FetchPaperRequest,
    ) -> Result<CallToolResult, McpError> {
        // Validate that at least one identifier is provided
        if !request.has_identifier() {
            return Err(McpError::invalid_params(
                "Either arxiv_id or url must be provided",
                None,
            ));
        }

        let client = create_paper_client();

        // Try to extract arxiv_id from various sources
        let arxiv_id = request
            .arxiv_id
            .clone()
            .or_else(|| request.url.as_ref().and_then(|u| extract_arxiv_id(u)));

        let paper = if let Some(ref arxiv_id) = arxiv_id {
            tracing::info!("Fetching paper by arXiv ID: {}", arxiv_id);

            client
                .fetch_by_arxiv_id(arxiv_id)
                .await
                .map_err(|e| McpError::internal_error(format!("Fetch failed: {}", e), None))?
        } else if let Some(ref url) = request.url {
            // Try to extract Semantic Scholar ID from URL
            if let Some(ss_id) = extract_ss_id(url) {
                tracing::info!("Fetching paper by Semantic Scholar ID: {}", ss_id);

                client
                    .fetch_by_ss_id(&ss_id)
                    .await
                    .map_err(|e| McpError::internal_error(format!("Fetch failed: {}", e), None))?
            } else {
                return Err(McpError::invalid_params(
                    format!(
                        "Unable to extract paper identifier from URL: {}. Supported URL formats: arXiv (arxiv.org/abs/*, arxiv.org/pdf/*), Semantic Scholar (semanticscholar.org/paper/*)",
                        url
                    ),
                    None,
                ));
            }
        } else {
            return Err(McpError::invalid_params("No identifier provided", None));
        };

        let response = FetchPaperResponse {
            paper: Paper {
                title: paper.title,
                authors: paper.authors.into_iter().map(|a| a.name).collect(),
                abstract_text: paper.abstract_text,
                arxiv_id: if paper.arxiv_id.is_empty() {
                    None
                } else {
                    Some(paper.arxiv_id)
                },
                ss_id: if paper.ss_id.is_empty() {
                    None
                } else {
                    Some(paper.ss_id)
                },
                categories: paper.categories,
                published_date: Some(paper.published_date.to_rfc3339()),
                pdf_url: if paper.url.is_empty() {
                    None
                } else {
                    Some(paper.url)
                },
                content: paper.extracted_text.map(|t| t.plain_text),
            },
        };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::internal_error(format!("Serialization error: {}", e), None))?;

        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

/// Extract arXiv ID from various URL formats
/// Supports:
/// - https://arxiv.org/abs/2301.00001
/// - https://arxiv.org/pdf/2301.00001.pdf
/// - http://arxiv.org/abs/2301.00001v1
fn extract_arxiv_id(url: &str) -> Option<String> {
    let url = url.trim();

    // Pattern for arxiv.org URLs
    if url.contains("arxiv.org") {
        // Try /abs/ format
        if let Some(pos) = url.find("/abs/") {
            let id_part = &url[pos + 5..];
            let id = id_part
                .split(|c| c == '?' || c == '#' || c == '/')
                .next()
                .unwrap_or(id_part);
            let id = strip_version_suffix(id);
            if !id.is_empty() {
                return Some(id.to_string());
            }
        }

        // Try /pdf/ format
        if let Some(pos) = url.find("/pdf/") {
            let id_part = &url[pos + 5..];
            let id = id_part
                .trim_end_matches(".pdf")
                .split(|c| c == '?' || c == '#' || c == '/')
                .next()
                .unwrap_or(id_part);
            let id = strip_version_suffix(id);
            if !id.is_empty() {
                return Some(id.to_string());
            }
        }
    }

    // Check if it's just an arxiv ID (e.g., "2301.00001")
    if url.chars().all(|c| c.is_ascii_digit() || c == '.') && url.contains('.') {
        return Some(url.to_string());
    }

    None
}

/// Strip version suffix like "v1", "v2" from arxiv ID
fn strip_version_suffix(id: &str) -> &str {
    // Look for pattern like "v" followed by digits at the end
    if let Some(v_pos) = id.rfind('v') {
        let after_v = &id[v_pos + 1..];
        // Check if everything after 'v' is digits
        if !after_v.is_empty() && after_v.chars().all(|c| c.is_ascii_digit()) {
            return &id[..v_pos];
        }
    }
    id
}

/// Extract Semantic Scholar paper ID from URL
/// Supports:
/// - https://www.semanticscholar.org/paper/Title-Name/abc123def456
/// - https://api.semanticscholar.org/CorpusID:12345678
fn extract_ss_id(url: &str) -> Option<String> {
    let url = url.trim();

    // Pattern for semanticscholar.org paper URLs
    if url.contains("semanticscholar.org/paper/") {
        // The ID is typically the last path segment
        let parts: Vec<&str> = url
            .trim_end_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        if let Some(id) = parts.last() {
            // Clean up query params
            let id = id.split('?').next().unwrap_or(id);
            if !id.is_empty() && id.len() >= 8 {
                return Some(id.to_string());
            }
        }
    }

    // Pattern for CorpusID
    if url.contains("CorpusID:") {
        if let Some(pos) = url.find("CorpusID:") {
            let id_part = &url[pos + 9..];
            let id = id_part
                .split(|c: char| !c.is_ascii_digit())
                .next()
                .unwrap_or(id_part);
            if !id.is_empty() {
                return Some(id.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_arxiv_id() {
        assert_eq!(
            extract_arxiv_id("https://arxiv.org/abs/2301.00001"),
            Some("2301.00001".to_string())
        );
        assert_eq!(
            extract_arxiv_id("https://arxiv.org/abs/2301.00001v2"),
            Some("2301.00001".to_string())
        );
        assert_eq!(
            extract_arxiv_id("https://arxiv.org/pdf/2301.00001.pdf"),
            Some("2301.00001".to_string())
        );
        assert_eq!(
            extract_arxiv_id("http://arxiv.org/abs/1706.03762"),
            Some("1706.03762".to_string())
        );
        assert_eq!(
            extract_arxiv_id("2301.00001"),
            Some("2301.00001".to_string())
        );
        assert_eq!(extract_arxiv_id("https://example.com"), None);
    }

    #[test]
    fn test_extract_ss_id() {
        assert_eq!(
            extract_ss_id("https://www.semanticscholar.org/paper/Attention-Is-All-You-Need/abc123def456"),
            Some("abc123def456".to_string())
        );
        assert_eq!(
            extract_ss_id("https://api.semanticscholar.org/CorpusID:12345678"),
            Some("12345678".to_string())
        );
        assert_eq!(extract_ss_id("https://example.com"), None);
    }
}
