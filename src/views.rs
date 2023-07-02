```rust
use tauri::Manager;
use crate::controllers::handle_message;
use crate::ascii_duck::render_duck;

pub fn setup_ui(manager: &Manager) {
    let window = manager.current_window();

    window.listen("tauri://event", move |event| {
        match event {
            tauri::event::Event::CloseRequested { label, .. } => {
                println!("Window close requested {}", label);
            }
            tauri::event::Event::Resized { label, .. } => {
                println!("Window resized {}", label);
            }
            tauri::event::Event::Moved { label, .. } => {
                println!("Window moved {}", label);
            }
            _ => (),
        }
    });

    window.listen("message", move |event| {
        if let Some(message) = event.payload().get("message") {
            handle_message(message);
        }
    });

    render_duck(&window);
}
```