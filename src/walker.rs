use crate::settings::Settings;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_excluded(e: &DirEntry, ex: &Settings) -> bool {
    if ex.except_dir.as_ref().unwrap().contains(
        &e.path()
            .parent()
            .unwrap_or(Path::new(""))
            .to_str()
            .unwrap()
            .to_string(),
    ) {
        return true;
    }
    if ex
        .except_path
        .as_ref()
        .unwrap()
        .contains(&e.path().to_str().unwrap().to_string())
    {
        return true;
    }
    if ex
        .except_filename
        .as_ref()
        .unwrap()
        .contains(&e.file_name().to_str().unwrap_or_default().to_string())
    {
        return true;
    }

    false
}

fn is_included(e: &DirEntry, inc: &Settings) -> bool {
    e.file_type().is_file()
        && inc.file_ext
            == e.file_name()
                .to_str()
                .unwrap_or_default()
                .split('.')
                .by_ref()
                .last()
                .unwrap()
}

fn is_hidden(e: &DirEntry) -> bool {
    e.path()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn walk(settings: &Settings) -> Vec<DirEntry> {
    let mut walker = WalkDir::new("./").into_iter();
    let mut targets: Vec<DirEntry> = vec![];

    loop {
        let entry: DirEntry = match walker.next() {
            None => return targets,
            Some(Err(err)) => panic!("ERROR: {}", err),
            Some(Ok(e)) => e,
        };

        if !is_hidden(&entry) && !is_excluded(&entry, &settings) {
            if entry.file_type().is_file() && is_included(&entry, &settings) {
                targets.push(entry);
            }
        }
    }
}
