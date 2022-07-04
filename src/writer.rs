use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use walkdir::{DirEntry};
use crate::processor::unfold;

fn needs_writing(path: &String, check_against: &String) -> bool {
    println!("{}", &path);
    match OpenOptions::new().read(true).open(&path) {
        Ok(mut target) => {

            let mut content = String::new();

            if let Err(err) = target.read_to_string(&mut content) {
                println!("Failed reading {}, due to:\n{:#?}\nContinuing...", &path, err);
            };

            check_against == &content
        },
        Err(err) => { if err.kind() == ErrorKind::NotFound { true } else { false }  }
    }
}

fn write_file(path: &String, content: &String) {
    let target = OpenOptions::new().truncate(true).create(true).write(true).open(&path);
    target.unwrap().write(content.as_bytes()).unwrap();
}

pub fn write(targets: &[DirEntry], out_path: &String) {
    for target in targets {
        if let Some(target_path) = target.path().to_str(){
            let processed = unfold(target_path).unwrap();

            let out_path = out_path.clone() + "/" + target.path().file_name().unwrap().to_str().unwrap();

            if needs_writing(&out_path, &processed) {
                write_file(&out_path, &processed);
            }
        };
    }
}
