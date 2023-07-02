```rust
use tauri::AppBuilder;
use crate::ui;
use crate::ascii_duck;
use crate::llm_chain;

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            match arg {
                "render_ascii_duck" => ascii_duck::render(),
                "talk_to_model" => llm_chain::talk_to_model(),
                _ => (),
            }
            Ok(())
        })
        .build(ui::create_window())
        .run();
}
```