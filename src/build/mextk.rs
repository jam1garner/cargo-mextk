use crate::paths::{PathExt, mextk_bin_dir};
use crate::Error;

use std::process::{Command, Stdio};
use std::io::Cursor;

#[cfg(unix)]
const MEXTK_FILE_NAME: &str = "MexTK";

#[cfg(not(unix))]
const MEXTK_FILE_NAME: &str = "MexTK.exe";

fn in_path(cmd: &str) -> bool {
    Command::new(cmd)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

pub fn command_recursive(try_again: bool) -> Result<Command, Error> {
    for cmd in &["mextk", "MexTK", "MexTk"] {
        if in_path(cmd) {
            return Ok(Command::new(cmd));
        }
    }

    let path = mextk_bin_dir().push_join(MEXTK_FILE_NAME);
    let dll_path = mextk_bin_dir().push_join("MexTK.dll");

    if path.exists() {
        Ok(Command::new(path))
    } else if dll_path.exists() {
        if in_path("dotnet") {
            let mut cmd = Command::new("dotnet");
            cmd.arg(dll_path);

            Ok(cmd)
        } else {
            Err(Error::NoDotNet)
        }
    } else if try_again {
        // Download
        download()?;

        // Try searching again
        command_recursive(false)
    } else {
        Err(Error::NoMextkInstalled(
            mextk_bin_dir().display().to_string()
        ))
    }
}

pub fn command() -> Result<Command, Error> {
    command_recursive(true)
}

const ZIP_URL: &str = "https://github.com/jam1garner/MexTK/releases/download/cross-platform/mextk.zip";

fn download() -> Result<(), Error> {
    println!("Downloading mextk...");
    let mut zip_contents = vec![];

    std::io::copy(
        &mut ureq::get(ZIP_URL).call().or(Err(Error::NetworkError))?.into_reader(),
        &mut zip_contents,
    ).unwrap();

    let mut zip = zip::ZipArchive::new(Cursor::new(zip_contents)).unwrap();

    zip.extract(mextk_bin_dir()).unwrap();

    Ok(())
}
