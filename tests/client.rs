mod constants;
use async_anthropic::client::AnthropicClient;
use async_anthropic::models::Model;
use async_anthropic::types::*;
use constants::CONSTITUTION;

/// this test is not idimpotent because claude caches can't be manually cleared.
#[tokio::test]
async fn test_full_caching() {
    // must have at least 1024 tokens for caching
    let request = CreateMessageRequest::builder()
            .model(Model::Claude37Sonnet)
            .system(Some(SystemPrompt::Content(vec![
                Content {
                    c_type: "text".to_string(),
                    text: "You are a learning tool designed to help people in tech understand government. Answer questions using the context provided".to_string(),
                    cache_control: None,
                },
                Content {
                    c_type: "text".to_string(),
                    text: CONSTITUTION.to_string(),
                    cache_control: Some(CacheControl::Ephemeral)
                }
            ])))
            .messages(vec![
                Message { role: "user".to_string(), content: "explain article 1 of the constitution".to_string() }
            ])
            .max_tokens(100)
            .build().unwrap();
    let client = AnthropicClient::new().unwrap();
    let response = client.messages().create(request, None).await.unwrap();
    let usage = response.usage;
    if usage.cache_creation_input_tokens.unwrap() == 0
        && usage.cache_read_input_tokens.unwrap() > 1024
    {
        // a cache line exists
        return;
    }
    let write_tokens = usage.cache_creation_input_tokens.unwrap();
    let request = CreateMessageRequest::builder()
            .model(Model::Claude37Sonnet)
            .system(Some(SystemPrompt::Content(vec![
                Content {
                    c_type: "text".to_string(),
                    text: "You are a learning tool designed to help people in tech understand government. Answer questions using the context provided".to_string(),
                    cache_control: None,
                },
                Content {
                    c_type: "text".to_string(),
                    text: CONSTITUTION.to_string(),
                    cache_control: Some(CacheControl::Ephemeral)
                }
            ])))
            .messages(vec![
                Message { role: "user".to_string(), content: "explain the 14th amendment to me".to_string() }
            ])
            .max_tokens(100)
            .build().unwrap();
    let response = client.messages().create(request, None).await.unwrap();
    let read_tokens = response.usage.cache_read_input_tokens.unwrap();
    assert_eq!(write_tokens, read_tokens);
    assert!(write_tokens > 1024);
}
