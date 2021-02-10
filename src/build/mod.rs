use crate::Error;
use cargo_metadata::Message;

use std::path::PathBuf;
use std::process::{Command, Stdio};

mod mextk;
mod dep_files;

const CARGO_OPTIONS: &[&str] = &[
    "rustc",
    "--message-format", "json-diagnostic-rendered-ansi",
    "--target", "powerpc-unknown-linux-gnu",
];

const RUSTC_OPTIONS: &[&str] = &[
    "-C", "relocation-model=static",
    "-C", "link-args=-MMD,-MP,-Wall,-DGEKKO,-mogc,-mcpu=750,-meabi,-mno-longcall,-mhard-float",
    "-C", "linker=powerpc-eabi-gcc",
    "--emit", "obj",
];

const RELEASE: &[&str] = &["--release", "--"];
const DEBUG: &[&str] = &["--"];

pub fn build(debug: bool) -> Result<PathBuf, Error> {
    let mut command =
        Command::new("cargo")
            .args(CARGO_OPTIONS)
            .args(if debug { DEBUG } else { RELEASE })
            .args(RUSTC_OPTIONS)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
    
    let last_artifact =
        cargo_metadata::parse_messages(command.stdout.as_mut().unwrap())
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|_| Error::FailParseCargoStream)?
            .into_iter()
            .filter_map(|message| {
                if let Message::CompilerArtifact(artifact) = message {
                    if ["mextk-sys", "mextk-libc"].contains(&&*artifact.target.name) {
                        None
                    } else {
                        artifact.filenames
                            .into_iter()
                            .filter_map(|mut path| {
                                let file_name = path.file_name()?.to_string_lossy().into_owned();
                                let file_name = file_name.strip_prefix("lib")?;

                                path.set_file_name(file_name);
                                path.set_extension("o");

                                if path.exists() {
                                    Some(path)
                                } else {
                                    None
                                }
                            })
                            .next()
                    }
                } else if let Message::CompilerMessage(message) = message {
                    if let Some(msg) = message.message.rendered {
                        println!("{}", msg);
                    }

                    None
                } else {
                    None
                }
            })
            .last()
            .ok_or(Error::NoBuildArtifact)?;

    let exit_status = command.wait().unwrap();

    if !exit_status.success() {
        return Err(Error::ExitStatus(exit_status.code().unwrap_or(1)))
    }

    let out_dat_folder = last_artifact.parent().unwrap();

    let dat_path = out_dat_folder.join("ftFunction.dat");

    //MexTK.exe -ff -i "whatever_your_file_is_named.c" -s ftFunction -o "ftFunction.dat"
    //              -t "ftFunction.txt" -l "melee.link" -q -ow -w -c
    let output = mextk::command()?
        .args(&["-ff", "-i"])
        .arg(&last_artifact)
        .args(&["-s", "ftFunction", "-o"])
        .arg(&dat_path)
        .arg("-t")
        .arg(dep_files::get("ftFunction.txt")?)
        .arg("-l")
        .arg(dep_files::get("melee.link")?)
        .args(&["-q", "-ow", "-w", "-c"])
        .status()
        .unwrap();

    if output.success() {
        Ok(last_artifact)
    } else {
        Err(Error::ExitStatus(output.code().unwrap())) 
    }
}
