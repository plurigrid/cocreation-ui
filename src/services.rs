```rust
use crate::llm_chain::LLMChain;
use crate::models::Message;
use crate::error::Result;
use y_rs::Doc;

pub struct Services {
    llm_chain: LLMChain,
    doc: Doc,
}

impl Services {
    pub fn new(llm_chain: LLMChain, doc: Doc) -> Self {
        Self { llm_chain, doc }
    }

    pub async fn send_message(&self, message: Message) -> Result<()> {
        let text = message.text.clone();
        self.llm_chain.send_message(text).await?;
        Ok(())
    }

    pub fn get_messages(&self) -> Vec<Message> {
        self.doc.get_messages()
    }
}
```