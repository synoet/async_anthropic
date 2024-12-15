use reqwest_eventsource::Error as EventsourceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid request: {message}")]
    InvalidRequest {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Authentication failed: {message}")]
    AuthenticationError {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Permission denied: {message}")]
    PermissionError {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Resource not found: {message}")]
    NotFound {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Request too large: {message}")]
    RequestTooLarge {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Internal server error: {message}")]
    InternalError {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Service overloaded: {message}")]
    ServiceOverloaded {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Network error: {inner}")]
    NetworkError {
        #[source]
        inner: reqwest::Error,
    },

    #[error("Unknown error: {message}")]
    Unknown {
        message: String,
        #[source]
        inner: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            match status.as_u16() {
                400 => ApiError::InvalidRequest {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                401 => ApiError::AuthenticationError {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                403 => ApiError::PermissionError {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                404 => ApiError::NotFound {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                413 => ApiError::RequestTooLarge {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                429 => ApiError::RateLimit {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                500 => ApiError::InternalError {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                529 => ApiError::ServiceOverloaded {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
                _ => ApiError::Unknown {
                    message: error.to_string(),
                    inner: Some(Box::new(error)),
                },
            }
        } else {
            ApiError::NetworkError { inner: error }
        }
    }
}

#[derive(Error, Debug)]
pub enum AnthropicError {
    #[error("failed to parse response from anthropic")]
    ParseError(String),
    #[error("failed to send request to anthropic")]
    ApiError(ApiError),
    #[error("error while streaming response from anthropic")]
    StreamError(StreamError),
    #[error("generic error")]
    GenericError(#[from] anyhow::Error),
    #[error("client error")]
    ClientError(#[from] ClientError),
    #[error("error while streaming response from anthropic")]
    EventsourceError(#[from] EventsourceError),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("missing api key env var, set ANTHROPIC_API_KEY")]
    MissingApiKey,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StreamErrorMessage {
    #[serde(rename = "type")]
    pub e_type: String,
    pub message: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct StreamError {
    #[serde(rename = "type")]
    pub e_type: String,
    pub error: StreamErrorMessage,
}
