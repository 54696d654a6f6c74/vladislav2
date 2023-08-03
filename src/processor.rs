use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn unfold(file_path: &str) -> std::io::Result<String> {
    let rx = Regex::new(r"<!--\s?include(?P<flag>\s\-\w+)?\s+(?P<path>[^\s]*)\s?-->").unwrap();

    let mut target = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    let mut html = String::new();
    let mut progress: usize = 0;

    if let Err(err) = target.read_to_string(&mut content) {
        panic!("{}", err);
    }

    for capture in rx.captures_iter(&content) {
        if let Some(include_line) = capture.get(0) {
            html += &content[progress..include_line.start()];
            progress = include_line.start() + include_line.as_str().len();
        }

        let path = &capture["path"];
        let mut template = String::new();

        if let Some(flag) = capture.name("flag") {
            if flag.as_str() == "r" {
                template = unfold(path)?;
            }
        }

        if let Ok(mut import_target) = File::open(path) {
            import_target.read_to_string(&mut template)?;
            html += &template;
        } else {
            println!("{} is not a valid path! Ignoring", path);
        };
    }

    html += &content[progress..];

    Ok(html)
}
