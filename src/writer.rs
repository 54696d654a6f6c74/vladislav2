use crate::processor::unfold;
use crate::settings::Settings;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use walkdir::DirEntry;

fn needs_writing(path: &String, check_against: &String) -> bool {
    match OpenOptions::new().read(true).open(path) {
        Ok(mut target) => {
            let mut content = String::new();

            if let Err(err) = target.read_to_string(&mut content) {
                println!(
                    "Failed reading {}, due to:\n{:#?}\nContinuing...",
                    &path, err
                );
            };

            !check_against.eq(&content)
        }
        Err(err) => {
            return err.kind() == ErrorKind::NotFound;
        }
    }
}

fn write_file(path: &String, content: &String) {
    let target = OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(path);
    target.unwrap().write_all(content.as_bytes()).unwrap();
}

pub fn write(targets: &[DirEntry], settings: &Settings) {
    for target in targets {
        if let Some(target_path) = target.path().to_str() {
            let processed = unfold(target_path, &settings.root_override).unwrap();

            let mut out_path = String::new();

            out_path.push_str(&settings.output_dir);
            out_path.push('/');
            out_path.push_str(target.path().file_stem().unwrap().to_str().unwrap());

            out_path.push('.');
            out_path.push_str(&settings.output_ext);

            if needs_writing(&out_path, &processed) {
                write_file(&out_path, &processed);
            }
        };
    }
}
