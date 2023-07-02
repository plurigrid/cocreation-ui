```rust
use tauri_build::{build_runtime, CustomRunner};

fn main() {
    build_runtime();

    let runner = CustomRunner::new()
        .before_run(|_| {
            println!("A unique ASCII duck is rendered briefly on each start");
            Ok(())
        });

    runner.run();
}
```