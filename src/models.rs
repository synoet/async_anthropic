use serde::{Deserialize, Serialize};
use strum_macros::IntoStaticStr;

#[derive(
    Serialize,
    Deserialize,
    strum_macros::Display,
    IntoStaticStr,
    strum_macros::EnumString,
    Clone,
    Debug,
)]
pub enum Model {
    #[serde(rename = "claude-3-5-sonnet-20241022")]
    #[strum(serialize = "claude-3-5-sonnet-20241022")]
    Claude35SonnetV2,
    #[serde(rename = "claude-3-5-sonnet@20230725")]
    #[strum(serialize = "claude-3-5-sonnet@20230725")]
    Claude35Sonnet,
    #[serde(rename = "claude-3-opus@20230725")]
    #[strum(serialize = "claude-3-opus@20230725")]
    Claude3Opus,
    #[serde(rename = "claude-3-haiku@20240307")]
    #[strum(serialize = "claude-3-haiku@20240307")]
    Claude3Haiku,
    #[serde(rename = "claude-3-sonnet@20240229")]
    #[strum(serialize = "claude-3-sonnet@20240229")]
    Claude3Sonnet,

    #[serde(rename = "claude-3-7-sonnet-20250219")]
    #[strum(serialize = "claude-3-7-sonnet-20250219")]
    Claude37Sonnet,
}

pub trait ModelVersion {
    fn without_version(&self) -> String;
    fn from_without_version(s: String) -> Self;
}

impl ModelVersion for Model {
    fn without_version(&self) -> String {
        match self {
            Model::Claude35SonnetV2 => "claude-3.5-sonnet-v2".to_string(),
            Model::Claude35Sonnet => "claude-3.5-sonnet".to_string(),
            Model::Claude3Opus => "claude-3-opus".to_string(),
            Model::Claude3Haiku => "claude-3-haiku".to_string(),
            Model::Claude3Sonnet => "claude-3-sonnet".to_string(),
            Model::Claude37Sonnet => "claude-3-7-sonnet".to_string(),
        }
    }

    fn from_without_version(s: String) -> Self {
        match s.as_str() {
            "claude-3.5-sonnet-v2" => Model::Claude35SonnetV2,
            "claude-3.5-sonnet" => Model::Claude35Sonnet,
            "claude-3-opus" => Model::Claude3Opus,
            "claude-3-haiku" => Model::Claude3Haiku,
            "claude-3-sonnet" => Model::Claude3Sonnet,
            "claude-3-7-sonnet" => Model::Claude37Sonnet,
            _ => Model::Claude37Sonnet,
        }
    }
}
