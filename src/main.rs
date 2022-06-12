use walkdir::{DirEntry};

mod settings;
mod processor;
mod walker;
mod writer;

const CONFIG_FILENAME: &str = "vlad_settings.json";

fn main() {
    let settings = settings::Settings::load_settings(CONFIG_FILENAME);

    let targets: Vec<DirEntry> = walker::walk(&settings);

    writer::write(targets, settings.output_dir);
}
