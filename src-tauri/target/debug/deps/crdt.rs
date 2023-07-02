```rust
use y_crdt::Doc;
use y_crdt::Text;

pub struct CRDT {
    doc: Doc,
    text: Text,
}

impl CRDT {
    pub fn new() -> Self {
        let doc = Doc::new();
        let text = doc.get_text("main");
        CRDT { doc, text }
    }

    pub fn insert(&mut self, index: usize, ch: char) {
        self.text.insert(index, ch);
    }

    pub fn delete(&mut self, index: usize) {
        self.text.delete(index);
    }

    pub fn get_content(&self) -> String {
        self.text.to_string()
    }

    pub fn apply_operation(&mut self, operation: Vec<u8>) {
        self.doc.apply_update(&operation);
    }

    pub fn generate_operation(&self) -> Vec<u8> {
        self.doc.encode_state_as_update()
    }
}
```