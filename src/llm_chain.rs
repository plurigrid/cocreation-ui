```rust
use llm::LanguageModel;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LlmChain {
    model: Arc<Mutex<LanguageModel>>,
}

impl LlmChain {
    pub fn new() -> Self {
        let model = Arc::new(Mutex::new(LanguageModel::new()));
        Self { model }
    }

    pub async fn process_input(&self, input: &str) -> String {
        let mut model = self.model.lock().await;
        model.process(input).await
    }
}
```