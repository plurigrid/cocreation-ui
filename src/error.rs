```rust
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    LlmChainError(String),
    CrdtError(String),
    UiError(String),
    GeneralError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::LlmChainError(e) => write!(f, "LLM Chain Error: {}", e),
            AppError::CrdtError(e) => write!(f, "CRDT Error: {}", e),
            AppError::UiError(e) => write!(f, "UI Error: {}", e),
            AppError::GeneralError(e) => write!(f, "General Error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
```