use ollama_rs::generation::chat::ChatMessage;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelRequestType<T: ToString + Display> {
    UnitTestWriteRequest(T),
    Inquiry(T),
}

impl<T> From<ModelRequestType<T>> for ChatMessage
where
    T: ToString + Display,
{
    fn from(value: ModelRequestType<T>) -> Self {
        let user_request = match value {
            ModelRequestType::UnitTestWriteRequest(s) => {
                format!("Write a unit test for the following code:\n```{0}```", s)
            }
            ModelRequestType::Inquiry(s) => s.to_string(),
        };
        ChatMessage::user(user_request)
    }
}
