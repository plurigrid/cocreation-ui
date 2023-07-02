```rust
use std::fs::File;
use std::io::prelude::*;

pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn render_ascii_duck() {
    let duck = include_str!("../ascii_duck.rs");
    println!("{}", duck);
}

pub fn send_message_to_llm_chain(message: &str) {
    let llm_chain = include_str!("../llm_chain.rs");
    // Assuming llm_chain has a function named `send_message`
    llm_chain::send_message(message);
}
```