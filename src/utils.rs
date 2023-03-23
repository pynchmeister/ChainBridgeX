use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigError {
    pub message: String,
}

pub fn read_file(path: &str) -> Result<String, ConfigError> {
    let file_path = Path::new(path);
    let display = file_path.display();
    let mut file = match File::open(&file_path) {
        Err(err) => {
            return Err(ConfigError {
                message: format!("Failed to open file {}: {}", display, err),
            });
        }
        Ok(file) => file,
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(ConfigError {
            message: format!("Failed to read file {}: {}", display, err),
        });
    }

    Ok(contents)
}

pub fn write_file(path: &str, contents: &str) -> Result<(), ConfigError> {
    let file_path = Path::new(path);
    let display = file_path.display();
    let mut file = match File::create(&file_path) {
        Err(err) => {
            return Err(ConfigError {
                message: format!("Failed to create file {}: {}", display, err),
            });
        }
        Ok(file) => file,
    };

    if let Err(err) = file.write_all(contents.as_bytes()) {
        return Err(ConfigError {
            message: format!("Failed to write to file {}: {}", display, err),
        });
    }

    Ok(())
}

