use crate::Error;

use std::process::{Command, Stdio};

pub fn run(debug: bool) -> Result<(), Error> {
    let mut command =
        Command::new("dolphin-emu")
            .args(&[""])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

    Err(todo!())
}
