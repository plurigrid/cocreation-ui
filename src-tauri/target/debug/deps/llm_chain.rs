```rust
use llm_chain::LLMChain;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LLMChainService {
    llm_chain: Arc<Mutex<LLMChain>>,
}

impl LLMChainService {
    pub fn new() -> Self {
        Self {
            llm_chain: Arc::new(Mutex::new(LLMChain::new())),
        }
    }

    pub async fn interact_with_model(&self, input: String) -> Result<String, llm_chain::Error> {
        let mut llm_chain = self.llm_chain.lock().await;
        llm_chain.interact(input).await
    }
}
```