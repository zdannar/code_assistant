use ollama_rs::generation::chat::ChatMessageResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelResponse {
    text: String,
}

impl Display for ModelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl From<ChatMessageResponse> for ModelResponse {
    fn from(value: ChatMessageResponse) -> Self {
        ModelResponse {
            text: value.message.content,
        }
    }
}
