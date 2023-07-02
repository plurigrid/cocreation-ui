```rust
use tauri::Manager;
use crate::ui;
use crate::ascii_duck;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // This code will be executed on application startup
            ascii_duck::render();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            ui::handle_input,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```