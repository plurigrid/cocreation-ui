```rust
use crate::services::LanguageModelService;
use crate::models::Message;
use crate::error::Result;
use tauri::Command;

#[derive(Default)]
pub struct Controller {
    language_model_service: LanguageModelService,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            language_model_service: LanguageModelService::new(),
        }
    }

    pub fn initialize_commands(self, app: &mut tauri::AppBuilder) {
        app.command("send_message", move |ctx, msg: Message| {
            let response = self.language_model_service.send_message(msg);
            match response {
                Ok(res) => Ok(res),
                Err(e) => Err(e.into()),
            }
        });
    }
}
```