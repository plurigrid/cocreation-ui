```rust
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub llm_chain: LLMChainConfig,
    pub crdt: CRDTConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMChainConfig {
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CRDTConfig {
    pub initial_state: HashMap<String, Arc<y_rs::YText>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UIConfig {
    pub ascii_duck: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            llm_chain: LLMChainConfig {
                endpoint: "http://localhost:8080".to_string(),
            },
            crdt: CRDTConfig {
                initial_state: HashMap::new(),
            },
            ui: UIConfig {
                ascii_duck: include_str!("../ascii_duck.rs"),
            },
        }
    }
}
```