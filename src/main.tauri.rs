```rust
use tauri::Manager;
use crate::ui::start_ui;
use crate::ascii_duck::render_duck;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let ascii_duck = render_duck();
            println!("{}", ascii_duck);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::controllers::handle_message,
            crate::services::talk_to_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```