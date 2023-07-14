use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn unfold(file_path: &str) -> std::io::Result<String> {
    let rx = Regex::new(r"<!--\s?include(\s\-\w+)?\s+([^\s]*)\s?-->").unwrap();

    let mut target = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    let mut html = String::new();
    let mut progress: usize = 0;

    if let Err(err) = target.read_to_string(&mut content) {
        panic!("{}", err);
    }

    while let Some(include_line) = rx.captures(&content) {
        if let Some(line) = include_line.get(0) {
            html += &content[progress..line.start()];
            progress = line.start() + line.as_str().len();
        };

        if let Some(path) = include_line.get(2) {
            if let Some(flag) = include_line.get(1) {
                let mut template = String::new();

                if flag.as_str() == "r" {
                    template = unfold(path.as_str())?;
                } else {
                    if let Ok(mut import_target) = File::open(path.as_str()) {
                        import_target.read_to_string(&mut template)?;
                    } else {
                        println!("{} is not a valid path! Ignoring", path.as_str());
                    };
                };

                html += &template;
            };
        }
    }

    html += &content[progress..];

    Ok(html)
}
