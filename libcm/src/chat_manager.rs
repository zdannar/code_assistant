use crate::{ModelRequestType, ModelResponse};
use ollama_rs::error::OllamaError;
use ollama_rs::generation::chat::{request::ChatMessageRequest, ChatMessage};
use ollama_rs::Ollama;
use std::fmt::Display;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct ChatManager {
    ollama: Ollama,
    model: String,
    // TODO: Change this;
    history: Vec<ChatMessage>,
}

#[derive(Debug, Error)]
pub enum ChatManagerError {
    #[error("transparent")]
    Ollama(#[from] OllamaError),
}

impl ChatManager {
    pub fn new<T: ToString + Display>(model: T, _system_prompt: Option<&str>) -> Self {
        ChatManager {
            ollama: Ollama::default(),
            model: model.to_string(),
            history: Vec::new(),
        }
    }

    pub async fn receive<T: ToString + Display>(
        &mut self,
        mrt: ModelRequestType<T>,
    ) -> Result<ModelResponse, ChatManagerError> {
        Ok(self
            .ollama
            .send_chat_messages_with_history(
                &mut self.history,
                ChatMessageRequest::new(self.model.clone(), vec![mrt.into()]),
            )
            .await?
            .into())
    }
}
