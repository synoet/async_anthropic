use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{constants::ANTHROPIC_VERSION, models::Model};

#[derive(Serialize, Deserialize, Clone, Debug)]
// TODO: support beta header
pub struct RequestConfig {
    /// The version of the Anthropic API you want to use.
    pub version: String,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            version: ANTHROPIC_VERSION.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum SystemPrompt {
    Text(String),
    Content(Vec<Content>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metadata {
    /// An external identifier for the user who is associated with the request.
    /// This should be a uuid, hash value, or other opaque identifier.
    /// Anthropic may use this id to help detect abuse.
    /// Do not include any identifying information such as name, email address, or phone number.
    user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
pub struct CreateMessageRequest {
    /// The model that will complete your prompt.
    /// See [Model] for additional details and options.
    pub model: Model,

    /// Input messages.
    /// Our models are trained to operate on alternating user and assistant conversational turns.
    /// When creating a new Message, you specify the prior conversational turns with the messages parameter, and the model then generates the next Message in the conversation. Consecutive user or assistant turns in your request will be combined into a single turn.
    /// Each input message must be an object with a role and content.
    /// You can specify a single user-role message, or you can include multiple user and assistant messages.
    /// If the final message uses the assistant role,
    /// the response content will continue immediately from the content in that message.
    /// This can be used to constrain part of the model's response.
    pub messages: Vec<Message>,

    /// The maximum number of tokens to generate before stopping.
    /// Note that our models may stop before reaching this maximum.
    /// This parameter only specifies the absolute maximum number of tokens to generate.
    pub max_tokens: u32,

    /// Custom text sequences that will cause the model to stop generating.
    /// Our models will normally stop when they have naturally completed their turn,
    /// which will result in a response stop_reason of "end_turn".
    ///
    /// If you want the model to stop generating when it encounters custom strings of text,
    /// you can use the stop_sequences parameter. If the model encounters one of the custom sequences,
    /// the response stop_reason value will be "stop_sequence" and the response stop_sequence value will
    /// contain the matched stop sequence.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,

    /// An object describing metadata about the request.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Whether to incrementally stream the response using server-sent events.
    #[builder(default)]
    pub stream: bool,

    /// A system prompt is a way of providing context and instructions to Claude,
    /// such as specifying a particular goal or role.

    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemPrompt>,

    /// Amount of randomness injected into the response.
    /// Defaults to 1.0. Ranges from 0.0 to 1.0. Use temperature closer to 0.0 for analytical / multiple choice,
    /// and closer to 1.0 for creative and generative tasks.
    /// Note that even with temperature of 0.0, the results will not be fully deterministic.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// Only sample from the top K options for each subsequent token.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,

    /// Use nucleus sampling.
    /// In nucleus sampling, we compute the cumulative distribution over all the options for
    /// each subsequent token in decreasing probability order and cut it off once it reaches a particular probability specified by top_p.
    /// You should either alter temperature or top_p, but not both.
    /// Recommended for advanced use cases only. You usually only need to use temperature.
    /// Only sample from the top K options for each subsequent token.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    // TODO: tool support
}

#[derive(Serialize, Deserialize, Clone, Builder, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    pub c_type: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum StopReason {
    /// the model reached a natural stopping point
    #[serde(rename = "end_turn")]
    EndTurn,
    /// we exceeded the requested max_tokens or the model's maximum
    #[serde(rename = "max_tokens")]
    MaxTokens,
    /// one of your provided custom stop_sequences was generated
    #[serde(rename = "max_time")]
    StopSequence,
    /// the model invoked one or more tools
    #[serde(rename = "tool_use")]
    ToolUse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Usage {
    /// The number of output tokens which were used.
    pub output_tokens: u32,
    /// The number of input tokens which were used.
    pub input_tokens: Option<u32>,
    /// The number of input tokens used to create the cache entry.
    pub cache_creation_input_tokens: Option<u32>,
    /// The number of input tokens read from the cache.
    pub cache_read_input_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Builder, Clone, Debug)]
pub struct CreateMessageResponse {
    /// Unique object identifier.
    /// The format and length of IDs may change over time.
    pub id: String,

    ///Object type.
    /// For Messages, this is always `message`.
    #[serde(rename = "type")]
    pub m_type: String,

    /// Conversational role of the generated message.
    /// This will always be "assistant".
    pub role: String,

    /// Content generated by the model.
    /// This is an array of content blocks, each of which has a type that determines its shape.
    pub content: Vec<Content>,

    /// The model that handled the request.
    pub model: Model,

    /// The reason that we stopped.
    /// This may be one the following values:
    /// "end_turn": the model reached a natural stopping point
    /// "max_tokens": we exceeded the requested max_tokens or the model's maximum
    /// "stop_sequence": one of your provided custom stop_sequences was generated
    /// "tool_use": the model invoked one or more tools
    /// In non-streaming mode this value is always non-null.
    /// In streaming mode, it is null in the message_start event and non-null otherwise.
    pub stop_reason: Option<StopReason>,

    /// Which custom stop sequence was generated, if any.
    /// This value will be a non-null string if one of your custom stop sequences was generated.
    pub stop_sequence: Option<String>,

    /// Billing and rate-limit usage.
    /// Anthropic's API bills and rate-limits by token counts, as tokens represent the underlying cost to our systems.
    /// Under the hood, the API transforms requests into a format suitable for the model. The model's output then goes through a parsing stage before becoming an API response. As a result, the token counts in usage will not match one-to-one with the exact visible content of an API request or response.
    /// For example, output_tokens will be non-zero, even for an empty string response from Claude.
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageDelta {
    pub stop_reason: Option<StopReason>,
    pub stop_sequence: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum CreateMessageStreamResponse {
    #[serde(rename = "message_start")]
    MessageStart { message: CreateMessageResponse },
    #[serde(rename = "content_block_start")]
    ContentBlockStart {
        index: Option<u32>,
        content_block: Content,
    },
    #[serde(rename = "content_block_delta")]
    ContentBlockDelta { index: Option<u32>, delta: Content },
    #[serde(rename = "content_block_stop")]
    ContentBlockStop { index: Option<u32> },
    #[serde(rename = "message_delta")]
    MessageDelta {
        index: Option<u32>,
        delta: MessageDelta,
        usage: Usage,
    },
    #[serde(rename = "message_stop")]
    MessageStop,
    #[serde(rename = "ping")]
    Ping,
}

impl CreateMessageRequest {
    pub fn builder() -> CreateMessageRequestBuilder {
        CreateMessageRequestBuilder::default()
    }
}
