```rust
use tauri::Manager;
use crate::ui::init_ui;
use crate::ascii_duck::show_ascii_duck;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            show_ascii_duck(&window);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::controllers::handle_messages
        ])
        .run(init_ui())
        .expect("error while running tauri application");
}
```