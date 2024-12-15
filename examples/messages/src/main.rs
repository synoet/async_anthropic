use anyhow::Result;
use async_anthropic::{
    client::AnthropicClient,
    models::Model,
    types::{CreateMessageRequest, Message},
};

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

    let res = client.messages().create(req, None).await?;

    println!("{:?}", res);

    Ok(())
}
