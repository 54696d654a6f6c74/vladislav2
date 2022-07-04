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

    let chunks = targets.chunks(2);
    
    for chunk in chunks {
        thread::spawn(|| {
            writer::write(&chunk, &settings.output_dir)
        });
    }
}
