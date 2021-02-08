use crate::Error;
use crate::paths::{dir_from_id, PathExt};

use std::process::{Command, Stdio};

pub fn run(debug: bool) -> Result<(), Error> {
    let id = "GALE01_v2";
    let path = dir_from_id(id).push_join("dolphin");

    if !path.exists() {
    }

    let mut command =
        Command::new("dolphin-emu")
            .args(&["-l", "-u"])
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    Err(todo!())
}
