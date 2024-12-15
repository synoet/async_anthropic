use anyhow::Result;

use crate::{
    error::{AnthropicError, ClientError},
    messages::Messages,
};
pub struct AnthropicClient {
    pub http_client: reqwest::Client,
    pub api_key: String,
}

impl AnthropicClient {
    pub fn new() -> Result<Self, AnthropicError> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| AnthropicError::ClientError(ClientError::MissingApiKey))?;

        Ok(Self {
            http_client: reqwest::Client::new(),
            api_key,
        })
    }

    pub fn with_api_key(api_key: String) -> Result<Self, ClientError> {
        Ok(Self {
            http_client: reqwest::Client::new(),
            api_key,
        })
    }

    pub fn messages(&self) -> Messages {
        Messages::new(self)
    }
}
