use crate::paths::{PathExt, mextk_bin_dir};
use crate::Error;

use std::process::{Command, Stdio};

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

pub fn command() -> Result<Command, Error> {
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
    } else {
        Err(Error::NoMextkInstalled(
            mextk_bin_dir().display().to_string()
        ))
    }
}
