use crate::settings::Settings;
use walkdir::{DirEntry, WalkDir};

fn is_excluded(file: &DirEntry, settings: &Settings) -> bool {
    let file_path = file.path();

    for component in file_path.components() {
        if String::from(component.as_os_str().to_str().unwrap_or("")).eq(&settings.output_dir) {
            return true;
        }
    }
    if let Some(except_dir) = settings.except_dir.as_ref() {
        for component in file_path.components() {
            if except_dir.contains(&String::from(component.as_os_str().to_str().unwrap_or(""))) {
                return true;
            }
        }
    }
    if let Some(except_path) = settings.except_path.as_ref() {
        if except_path.contains(&file_path.to_str().unwrap().to_string()) {
            return true;
        }
    }
    if let Some(except_filename) = settings.except_filename.as_ref() {
        if except_filename.contains(&file.file_name().to_str().unwrap_or_default().to_string()) {
            return true;
        }
    }

    return false;
}

fn is_eligible(file: &DirEntry, settings: &Settings) -> bool {
    fn get_file_ext(file: &DirEntry) -> &str {
        return file
            .file_name()
            .to_str()
            .unwrap_or_default()
            .split('.')
            .by_ref()
            .last()
            .unwrap_or_default();
    }

    return file.file_type().is_file() && settings.file_ext.eq(get_file_ext(file));
}

fn is_hidden(file: &DirEntry) -> bool {
    return file
        .path()
        .file_name()
        .unwrap_or_default()
        .to_str()
        .map(|s| return s.starts_with('.'))
        .unwrap_or(false);
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

        if !is_hidden(&entry) && !is_excluded(&entry, settings) && is_eligible(&entry, settings) {
            targets.push(entry);
        }
    }
}
