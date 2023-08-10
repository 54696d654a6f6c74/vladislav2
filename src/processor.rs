use regex::Regex;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn unfold(file_path: &str, root_override: &Option<String>) -> std::io::Result<String> {
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
            progress = include_line.end();
        }

        let path = get_path(&capture["path"], root_override);
        let mut template = String::new();

        if let Some(flag) = capture.name("flag") {
            if flag.as_str() == "r" {
                template = unfold(&path, root_override)?;
            }
        }

        if let Ok(mut import_target) = File::open(&path) {
            import_target.read_to_string(&mut template)?;
            html += &template;
        } else {
            println!("{} is not a valid path! Ignoring", &path);
        };
    }

    html += &content[progress..];

    return Ok(html);
}

fn get_path(path: &str, root_override: &Option<String>) -> String {
    let mut built_path = String::new();

    if let Some(r_override) = root_override {
        built_path.push_str(r_override);
    }

    built_path.push_str(path);

    return built_path;
}
