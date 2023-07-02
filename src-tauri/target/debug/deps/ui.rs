```rust
use tauri::Manager;
use crate::ascii_duck::render_duck;
use crate::llm_chain::talk_to_model;

#[tauri::command]
async fn render_ui(window: tauri::Window) {
    let duck = render_duck();
    window.emit("renderDuck", duck).unwrap();

    let model_response = talk_to_model("Hello, model!");
    window.emit("modelResponse", model_response).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![render_ui])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```