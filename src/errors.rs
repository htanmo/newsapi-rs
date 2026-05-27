use thiserror::Error;

#[derive(Error, Debug)]
pub enum NewsApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API returned error: {code} - {message}")]
    Api { code: String, message: String },

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Invalid Parameters: {0}")]
    InvalidParams(String),

    #[error("Json deserialization error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, NewsApiError>;
