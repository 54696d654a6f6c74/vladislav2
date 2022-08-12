use std::thread;
use walkdir::{DirEntry};

mod settings;
mod processor;
mod walker;
mod writer;

const CONFIG_FILENAME: &str = "vlad_settings.json";

fn main() {
    let settings = settings::Settings::load_settings(CONFIG_FILENAME);

    let targets: Vec<DirEntry> = walker::walk(&settings);

    let mut groups = vec![];

    for chunk in targets.chunks(2) {
        groups.push(chunk.to_owned());
    }

    for chunk in groups {
        let output_dir = settings.output_dir.clone();
        thread::spawn(move || {
            writer::write(&chunk, &output_dir);
            chunk
        });
    };
}
