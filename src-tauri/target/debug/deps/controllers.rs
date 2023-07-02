```rust
use tauri::Manager;
use crate::services::llm_service;
use crate::models::Message;

pub struct Controller {
    llm_service: llm_service::LLMService,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            llm_service: llm_service::LLMService::new(),
        }
    }

    pub fn handle_message(&self, message: Message, window: tauri::Window) {
        match message {
            Message::TalkToModel { content } => {
                let response = self.llm_service.talk_to_model(&content);
                window.emit("modelResponse", response).expect("failed to emit");
            }
            _ => (),
        }
    }
}
```