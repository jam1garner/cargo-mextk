use crate::paths::{dir_from_id, PathExt};
use crate::manifest::Manifest;
use crate::{install, Error};

use std::{io, fs};
use std::path::Path;
use std::process::Command;

use include_dir::{include_dir, Dir};

pub fn run(_debug: bool, no_restore: bool) -> Result<(), Error> {
    let toml = Manifest::from_current_directory()?;
    let id = toml.game_id.as_deref().unwrap_or("GALE01_v2");

    let iso_dir = dir_from_id(id);
    if !iso_dir.exists() {
        return Err(Error::NoSuchIso)
    }

    let sys_dir = iso_dir.join("extracted").push_join("sys");
    let dolphin_dir = iso_dir.join("dolphin");

    if !dolphin_dir.exists() {
        create_dolphin_dir(&dolphin_dir, &sys_dir);
    }

    install(!no_restore).unwrap();

    Command::new("dolphin-emu")
        .args(&["-l", "-u"])
        .arg(dolphin_dir)
        .arg("-e")
        .arg(sys_dir.push_join("main.dol"))
        .status()
        .unwrap();

    Ok(())
}

const DOLPHIN_TEMPLATE: Dir = include_dir!("src/dolphin_template");

fn create_dolphin_dir(path: &Path, iso_path: &Path) {
    let _ = extract_recursive(path, iso_path, &DOLPHIN_TEMPLATE);
}

fn extract_recursive(to: &Path, iso_dir: &Path, dir: &Dir) -> io::Result<()> {
    if !to.exists() {
        fs::create_dir_all(to)?;
    }

    for file in dir.files() {
        match file.path() {
            path if path.to_string_lossy().contains("Dolphin.ini") => {
                fs::write(
                    to.join(path),
                    file.contents_utf8()
                        .unwrap()
                        .replace("{{iso_path}}", iso_dir.to_string_lossy().as_ref())
                )?
            }
            relative_path => fs::write(to.join(relative_path), file.contents())?
        }
    }

    for child_dir in dir.dirs() {
        fs::create_dir_all(to.join(child_dir.path()))?;
        extract_recursive(to, iso_dir, child_dir)?;
    }

    Ok(())
}
