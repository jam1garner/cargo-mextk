use std::fs;
use crate::Error;
use crate::paths::{dir_from_id, PathExt};
use include_dir::{Dir, File, include_dir};

const ADDITIONS: Dir = include_dir!("src/mex-additions");

pub fn add(id: &str) -> Result<(), Error> {
    let dir = dir_from_id(id).push_join("extracted");

    let files = get_files(&ADDITIONS);

    for file in files {
        let path = dir.join(file.path);

        fs::write(path, file.contents()).unwrap();
    }

    Ok(())
}

fn get_files<'a>(dir: &'a Dir) -> Vec<File<'a>> {
    if dir.dirs().len() == 0 {
        dir.files.to_owned()
    } else {
        dir.dirs()
            .iter()
            .map(|dir| get_files(dir).into_iter())
            .flatten()
            .chain(dir.files().iter().map(|x| *x))
            .collect()
    }
}
