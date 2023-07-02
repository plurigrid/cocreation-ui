```rust
use std::fs;
use std::io::Error;

pub fn read_file(path: &str) -> Result<String, Error> {
    fs::read_to_string(path)
}

pub fn write_file(path: &str, contents: &str) -> Result<(), Error> {
    fs::write(path, contents)
}

pub fn append_to_file(path: &str, contents: &str) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", contents)
}

pub fn delete_file(path: &str) -> Result<(), Error> {
    fs::remove_file(path)
}
```