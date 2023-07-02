```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub connected: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub users: Vec<User>,
    pub messages: Vec<Message>,
}

impl User {
    pub fn new(id: String, username: String) -> Self {
        User {
            id,
            username,
            connected: false,
        }
    }
}

impl Message {
    pub fn new(id: String, user_id: String, content: String, timestamp: i64) -> Self {
        Message {
            id,
            user_id,
            content,
            timestamp,
        }
    }
}

impl Session {
    pub fn new(id: String) -> Self {
        Session {
            id,
            users: Vec::new(),
            messages: Vec::new(),
        }
    }
}
```