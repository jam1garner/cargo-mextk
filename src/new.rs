use crate::Error;
use include_dir::{include_dir, Dir};

use std::{io, fs};
use std::path::Path;

const TEMPLATE: Dir = include_dir!("src/template");
const TEMPLATE_GIT: Dir = include_dir!("src/template_git");

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
        match file.path() {
            path if path.to_string_lossy() == "_Cargo.toml" => {
                fs::write(
                    to.join("Cargo.toml"),
                    file.contents_utf8()
                        .unwrap()
                        .replace("{{template_name}}", to.to_string_lossy().as_ref())
                )?
            }
            relative_path => fs::write(to.join(relative_path), file.contents())?
        }
    }

    for child_dir in dir.dirs() {
        fs::create_dir_all(to.join(child_dir.path()))?;
        extract_recursive(to, child_dir)?;
    }

    Ok(())
}
