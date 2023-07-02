```rust
use tauri::AppBuilder;
use crate::ui::init_ui;
use crate::ascii_duck::show_ascii_duck;
use crate::llm_chain::init_llm_chain;
use crate::crdt::init_crdt;

pub fn run_app() {
    AppBuilder::new()
        .invoke_handler(tauri::generate_handler![init_ui, init_llm_chain, init_crdt])
        .build(tauri::generate_context!())
        .unwrap()
        .run(|app| {
            show_ascii_duck();
            tauri::event::emit(&app, String::from("app-ready"), Some(String::from("App is ready"))).unwrap();
        });
}
```