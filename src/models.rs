use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub content: Option<String>,
    pub url: String,
    #[serde(rename = "urlToImage")]
    pub url_to_image: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum ApiResponse {
    #[serde(rename = "ok")]
    Success(SuccessResponse),
    #[serde(rename = "error")]
    Error(ErrorResponse),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    #[serde(rename = "totalResults")]
    pub total_results: u32,
    pub articles: Vec<Article>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "status")]
pub enum SourcesResponse {
    #[serde(rename = "ok")]
    Ok { sources: Vec<SourceDetail> },
    #[serde(rename = "error")]
    Err(ErrorResponse),
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SourceDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub category: String,
    pub language: String,
    pub country: String,
}
