use anyhow::Result;
use std::pin::Pin;

use crate::{
    client::AnthropicClient,
    constants::{ANTHROPIC_BASE_URL, API_KEY_HEADER, VERSION_HEADER},
    error::{AnthropicError, StreamError},
    types::{
        CreateMessageRequest, CreateMessageResponse, CreateMessageStreamResponse, RequestConfig,
    },
};
use futures::{stream::StreamExt, Stream};
use reqwest_eventsource::{Event, RequestBuilderExt};
use tokio::sync::mpsc;

pub struct Messages<'c> {
    client: &'c AnthropicClient,
}

impl<'c> Messages<'c> {
    pub fn new(client: &'c AnthropicClient) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: CreateMessageRequest,
        config: Option<RequestConfig>,
    ) -> Result<CreateMessageResponse, AnthropicError> {
        let url = format!("{}/v1/messages", ANTHROPIC_BASE_URL.to_string());
        let client = self.client.http_client.clone();

        let config = config.unwrap_or_default();

        let mut request = request;
        request.stream = false;

        let res = match client
            .post(&url)
            .header("content-type", "application/json; charset=utf-8")
            .header(API_KEY_HEADER, self.client.api_key.clone())
            .header(VERSION_HEADER, config.version.clone())
            .json(&request)
            .send()
            .await
        {
            Ok(res) => res,
            Err(e) => {
                tracing::error!(error=?e, "failed to send request to anthropic");
                return Err(AnthropicError::ApiError(e.into()));
            }
        };

        let json = res.json::<serde_json::Value>().await.map_err(|e| {
            tracing::error!(error=?e, "failed to parse response from anthropic");
            AnthropicError::ParseError(e.to_string())
        })?;

        let response = serde_json::from_value::<CreateMessageResponse>(json)
            .map_err(|e| AnthropicError::ParseError(format!("failed to parse response: {}", e)))?;

        Ok(response)
    }

    pub async fn create_stream(
        &self,
        request: CreateMessageRequest,
        config: Option<RequestConfig>,
    ) -> Result<
        Pin<
            Box<
                dyn Stream<Item = Result<CreateMessageStreamResponse, AnthropicError>>
                    + Send
                    + 'static,
            >,
        >,
        AnthropicError,
    > {
        let mut request = request;
        request.stream = true;
        // Clone the required values before moving into the spawned task
        let client = self.client.http_client.clone();
        let config = config.unwrap_or_default();
        let api_key = self.client.api_key.clone();
        let (wx, rx) = mpsc::unbounded_channel();

        dbg!(&request);

        tokio::spawn(async move {
            let url = format!("{}/v1/messages", ANTHROPIC_BASE_URL.to_string());
            let mut source = match client
                .post(&url)
                .header("content-type", "application/json; charset=utf-8")
                .header(API_KEY_HEADER, &api_key)
                .header(VERSION_HEADER, config.version.clone())
                .json(&request)
                .eventsource()
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!(error=?e, "failed to send request to anthropic");
                    return Err(AnthropicError::GenericError(anyhow::anyhow!(e)));
                }
            };

            while let Some(sse_event) = source.next().await {
                let event = match sse_event {
                    Ok(e) => match e {
                        Event::Message(m) => m,
                        Event::Open => {
                            continue;
                        }
                    },
                    Err(e) => {
                        tracing::error!(error=?e, "failed to send request to anthropic");
                        return Err(AnthropicError::EventsourceError(e));
                    }
                };

                let message = event.data;
                let res = match serde_json::from_str::<CreateMessageStreamResponse>(&message) {
                    Ok(c) => c,
                    Err(parse_error) => match serde_json::from_str::<StreamError>(&message) {
                        Ok(c) => {
                            tracing::error!(error=?c, "stream raw predict failed");
                            if let Err(send_error) =
                                wx.send(Err(AnthropicError::StreamError(c.into())))
                            {
                                tracing::error!(
                                    error=?send_error,
                                    "failed to send error message to stream"
                                );
                                break;
                            }
                            continue;
                        }
                        Err(_) => {
                            tracing::error!(error=?parse_error, "failed to parse error response from claude");
                            if let Err(send_error) =
                                wx.send(Err(AnthropicError::ParseError(parse_error.to_string())))
                            {
                                tracing::error!(error=?send_error, "failed to send error message to stream");
                                break;
                            }
                            continue;
                        }
                    },
                };

                let is_stop = matches!(res, CreateMessageStreamResponse::MessageStop);
                if let Err(send_error) = wx.send(Ok(res)) {
                    tracing::error!(error=?send_error, "failed to send response to stream");
                    break;
                }
                if is_stop {
                    break;
                }
            }
            Ok(())
        });

        Ok(Box::pin(
            tokio_stream::wrappers::UnboundedReceiverStream::new(rx),
        ))
    }
}
