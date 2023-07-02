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
    pub timestamp: u64,
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

    pub fn connect(&mut self) {
        self.connected = true;
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }
}

impl Message {
    pub fn new(id: String, user_id: String, content: String, timestamp: u64) -> Self {
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

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}
```