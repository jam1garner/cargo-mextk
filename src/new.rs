use crate::Error;
use include_dir::{include_dir, Dir};

use std::fs;
use std::io;
use std::path::Path;

const TEMPLATE: Dir = include_dir!("template");
const TEMPLATE_GIT: Dir = include_dir!("template_git");

pub fn new(name: &str) -> Result<(), Error> {
    let new_dir = Path::new(name);

    if new_dir.exists() {
        Err(Error::DirAlreadyExists)
    } else {
        extract_recursive(new_dir, &TEMPLATE)?;
        extract_recursive(&new_dir.join(".git"), &TEMPLATE_GIT)?;

        Ok(())
    }
}

fn extract_recursive(to: &Path, dir: &Dir) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    for file in dir.files() {
        fs::write(to.join(file.path()), file.contents())?;
    }

    for child_dir in dir.dirs() {
        extract_recursive(to, child_dir)?;
    }

    Ok(())
}
