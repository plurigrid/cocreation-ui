```rust
use tauri::Manager;
use crate::controllers::handle_message;
use crate::ascii_duck::render_duck;

pub fn init_views<M: Manager>(app: &mut M) {
    let main_window = app.get_window("main").unwrap();

    main_window.listen("render_duck", |_| {
        render_duck();
    });

    main_window.listen("message", |event| {
        let (message, callback) = event.payload().unwrap();
        handle_message(message, callback);
    });
}
```