use std::thread;
use walkdir::DirEntry;

mod processor;
mod settings;
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

    let mut threads = vec![];

    for chunk in groups {
        let output_dir = settings.output_dir.clone();
        let output_ext = settings.output_ext.clone();
        let handle = thread::spawn(move || {
            writer::write(&chunk, &output_dir, &output_ext);
            chunk
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
