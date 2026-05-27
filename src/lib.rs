mod errors;
mod models;
mod params;

use std::time::Duration;

use reqwest::Client as HttpClient;

pub use crate::{
    errors::{NewsApiError, Result},
    models::{ApiResponse, SourceDetail, SourcesResponse, SuccessResponse},
    params::{EverythingParams, SourceParams, TopHeadlinesParams},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApiVersion {
    #[default]
    V2,
}

impl ApiVersion {
    fn path(&self) -> &'static str {
        match self {
            ApiVersion::V2 => "/v2",
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiVersion::V2 => write!(f, "v2"),
        }
    }
}

#[derive(Debug)]
pub struct NewsApiClient {
    api_key: String,
    base_url: String,
    http: HttpClient,
    version: ApiVersion,
}

impl NewsApiClient {
    const BASE_URL: &str = "https://newsapi.org";

    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_version(api_key, ApiVersion::default())
    }

    pub fn with_version(api_key: impl Into<String>, version: ApiVersion) -> Self {
        let http = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key: api_key.into(),
            base_url: format!("{}{}", Self::BASE_URL, version.path()),
            http: http,
            version,
        }
    }

    pub fn set_version(&mut self, version: ApiVersion) {
        self.version = version;
        self.base_url = format!("{}{}", Self::BASE_URL, version.path());
    }

    pub fn version(&self) -> ApiVersion {
        self.version
    }

    async fn handle_response(&self, response: reqwest::Response) -> Result<SuccessResponse> {
        let body = response.bytes().await?;
        let api_response: ApiResponse = serde_json::from_slice(&body)?;
        match api_response {
            ApiResponse::Success(success) => Ok(success),
            ApiResponse::Error(error) => {
                if error.code == "rateLimited" {
                    Err(NewsApiError::RateLimitExceeded)
                } else if error.code == "apiKeyInvalid" {
                    Err(NewsApiError::InvalidApiKey)
                } else {
                    Err(NewsApiError::Api {
                        code: error.code,
                        message: error.message,
                    })
                }
            }
            ApiResponse::Unknown => Err(NewsApiError::InvalidParams(
                "Unexpected response format".into(),
            )),
        }
    }

    pub async fn top_headlines(&self, params: &TopHeadlinesParams) -> Result<SuccessResponse> {
        let mut query = vec![("apiKey", self.api_key.to_string())];

        if let Some(country) = &params.country {
            query.push(("country", country.to_string()));
        }

        if let Some(category) = &params.category {
            query.push(("category", category.to_string()));
        }

        if let Some(sources) = &params.sources {
            query.push(("sources", sources.to_string()));
        }

        if let Some(q) = &params.q {
            query.push(("q", q.to_string()));
        }

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let url = format!("{}/top-headlines", self.base_url);
        let response = self.http.get(&url).query(&query).send().await?;

        self.handle_response(response).await
    }

    pub async fn everything(&self, params: &EverythingParams) -> Result<SuccessResponse> {
        let mut query = vec![("apiKey", self.api_key.to_string())];
        if let Some(q) = &params.q {
            query.push(("q", q.to_string()));
        }

        if let Some(q_in_title) = &params.q_in_title {
            query.push(("qInTitle", q_in_title.to_string()));
        }

        if let Some(sources) = &params.sources {
            query.push(("sources", sources.to_string()));
        }

        if let Some(domains) = &params.domains {
            query.push(("domains", domains.to_string()));
        }

        if let Some(from) = &params.from {
            query.push(("from", from.to_string()));
        }

        if let Some(to) = &params.to {
            query.push(("to", to.to_string()));
        }

        if let Some(language) = &params.language {
            query.push(("language", language.to_string()));
        }

        if let Some(sort_by) = &params.sort_by {
            query.push(("sortBy", sort_by.to_string()));
        }

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let url = format!("{}/everything", self.base_url);
        let response = self.http.get(&url).query(&query).send().await?;
        self.handle_response(response).await
    }

    pub async fn sources(&self, params: &SourceParams) -> Result<Vec<SourceDetail>> {
        let mut query = vec![("apiKey", self.api_key.to_string())];

        if let Some(category) = &params.category {
            query.push(("category", category.to_string()));
        }

        if let Some(language) = &params.language {
            query.push(("language", language.to_string()));
        }

        if let Some(country) = &params.country {
            query.push(("country", country.to_string()));
        }

        let url = format!("{}/sources", self.base_url);
        let response = self.http.get(&url).query(&query).send().await?;
        let body = response.bytes().await?;

        let source_resp: SourcesResponse = serde_json::from_slice(&body)?;
        match source_resp {
            SourcesResponse::Ok { sources } => Ok(sources),
            SourcesResponse::Err(err) => Err(NewsApiError::Api {
                code: err.code,
                message: err.message,
            }),
            SourcesResponse::Unknown => Err(NewsApiError::InvalidParams(
                "Unknown response from sources endpoint".into(),
            )),
        }
    }
}
