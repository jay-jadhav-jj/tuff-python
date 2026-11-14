use std::{fs, path::Path};

pub fn read_file(path: impl AsRef<Path>) -> Result<String, String> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    fs::read_to_string(path).map_err(|err| format!("Failed to read '{}': {}", path.display(), err))
}

pub fn write_file(path: impl AsRef<Path>, contents: &str) -> Result<(), String> {
    let path = path.as_ref();

    fs::write(path, contents)
        .map_err(|err| format!("Failed to write '{}': {}", path.display(), err))
}
