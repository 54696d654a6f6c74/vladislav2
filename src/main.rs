#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

use settings::Settings;
use std::thread::{self};
use walkdir::DirEntry;

mod processor;
mod settings;
mod walker;
mod writer;

const CONFIG_FILENAME: &str = "vlad_settings.json";

fn main() {
    let settings = settings::Settings::load_settings(CONFIG_FILENAME);

    let targets: Vec<DirEntry> = walker::walk(&settings);

    let thread_count = targets.len() / settings.files_per_thread as usize;

    let mut groups = vec![];

    for chunk in targets.chunks(if thread_count > 0 { thread_count } else { 1 }) {
        groups.push(chunk.to_owned());
    }

    spawn_threads(&groups, &settings);
}

fn spawn_threads(groups: &[Vec<DirEntry>], settings: &Settings) {
    return thread::scope(|s| {
        for chunk in groups.iter() {
            s.spawn(move || writer::write(chunk, settings));
        }
    });
}
