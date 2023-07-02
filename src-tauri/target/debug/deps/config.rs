```rust
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub ascii_duck: String,
    pub llm_chain_config: LLMChainConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMChainConfig {
    pub model_path: String,
    pub model_params: HashMap<String, String>,
}

impl AppConfig {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            ascii_duck: String::from("ascii_duck.rs"),
            llm_chain_config: LLMChainConfig {
                model_path: String::from("llm_chain.rs"),
                model_params: HashMap::new(),
            },
        })
    }
}
```