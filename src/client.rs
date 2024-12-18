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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Model;
    use crate::types::{CreateMessageRequest, Message, StopReason, Tool, ToolChoice};
    use anyhow::Result;
    use serde_json::json;

    #[tokio::test]
    async fn test_tool_use() -> Result<()> {
        let client = AnthropicClient::new()?;
        let request = CreateMessageRequest::builder()
            .model(Model::Claude3Opus)
            .messages(vec![Message {
                role: "user".to_string(),
                content: "What is 123 + 456?".to_string(),
            }])
            .max_tokens(1024)
            .tool_choice(Some(ToolChoice::Any {
                disable_parallel_tool_use: true,
            }))
            .tools(Some(vec![Tool {
                name: "calculator".to_string(),
                description: Some("A calculator that can perform basic arithmetic".to_string()),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "result": {
                            "type": "number",
                            "description": "the result of the mathematical expression proposed by the user"
                        }
                    },
                    "required": ["result"]
                }),
            }]))
            .build()?;

        // println!("REQUEST JSON {:#?}", serde::json!(""));

        let response = client.messages().create(request, None).await?;
        println!("Tool Response: {:#?}", response);
        // Check if the response contains tool calls
        assert!(response.stop_reason == Some(StopReason::ToolUse));
        Ok(())
    }
}
