```rust
use tauri::Manager;
use crate::ascii_duck::render_duck;
use crate::llm_chain::talk_to_model;

pub fn create_ui() {
    tauri::Builder::default()
        .on_page_load(|window, _| {
            let _ = window.eval("document.getElementById('ascii-duck').innerHTML = render_duck();");
            let _ = window.eval("document.getElementById('talk-to-model').addEventListener('input', function(event) {
                talk_to_model(event.target.value);
            });");
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn render_duck() -> String {
    render_duck()
}

fn talk_to_model(input: String) {
    talk_to_model(input);
}
```