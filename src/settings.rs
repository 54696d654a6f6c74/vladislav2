use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub file_ext: String,
    pub output_dir: String,
    pub output_ext: String,
    pub files_per_thread: u16,
    pub except_path: Option<Vec<String>>,
    pub except_dir: Option<Vec<String>>,
    pub except_filename: Option<Vec<String>>
}

impl Settings {
    pub fn new(data: &str) -> Result<Self> {
        Ok(serde_json::from_str(data)?)
    }

    fn stringified(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn load_settings(path: &str) -> Settings {
        let mut target = match OpenOptions::new()
            .read(true)
            .open(path) {
                Err(_) => {
                    let mut target = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(path)
                        .unwrap();
                    let content = Settings::default().stringified().unwrap();

                    target.write(&content.as_bytes()).unwrap();

                    return Settings::default();
                }
                Ok(val) => val
            };

        let mut settings_read = String::new();
        target.read_to_string(&mut settings_read).unwrap(); // We want to panic on failure here

        Settings::new(&settings_read).unwrap() // Similarly, we're OK to panic here
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            file_ext: String::from("vlad"),
            output_dir: String::from("./vlad_out"),
            output_ext: String::from("html"),
            files_per_thread: 15,
            except_dir: Some(vec![String::from("templates")]),
            except_path: Some(vec!()),
            except_filename: Some(vec!())
        }
    }
}
