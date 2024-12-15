use anyhow::Result;
use async_anthropic::{
    client::AnthropicClient,
    models::Model,
    types::{CreateMessageRequest, Message},
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let client = AnthropicClient::new()?;

    let req = CreateMessageRequest::builder()
        .model(Model::Claude35SonnetV2)
        .max_tokens(812)
        .messages(vec![Message {
            role: "user".to_string(),
            content: "please write me a sonnet".to_string(),
        }])
        .build()?;

    let mut res = client.messages().create_stream(req, None).await?;

    while let Some(msg) = res.next().await {
        match msg {
            Ok(msg) => {
                dbg!(msg);
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }

    Ok(())
}
