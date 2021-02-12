use cargo_metadata::Message;
use crate::paths::extracted_dat_path;
use crate::{Error, manifest::Manifest};

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use owo_colors::OwoColorize;

pub mod mextk;
mod dep_files;

const CARGO_OPTIONS: &[&str] = &[
    "+nightly",
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
    ensure_nightly_ppc_installed();

    let toml = Manifest::from_current_directory()?;

    let dat_name = toml.dat.ok_or(Error::NoDatName)?;

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

    let dat_path = out_dat_folder.join(&dat_name);

    let id = "GALE01_v2";
    let original_dat_path = extracted_dat_path(id, &dat_name);

    fs::copy(&original_dat_path, &dat_path).unwrap();

    let symbols = toml.symbols
        .as_deref()
        .map(to_file_name)
        .unwrap_or_else(|| {
            println!("{}", "Warning: defaulting symbols to `fighter` ".bright_yellow());

            Ok("ftFunction.txt")
        })?;

    //MexTK.exe -ff -i "whatever_your_file_is_named.c" -s ftFunction -o "ftFunction.dat"
    //              -t "ftFunction.txt" -l "melee.link" -q -ow -w -c
    let output = mextk::command()?
        .args(&["-ff", "-i"])
        .arg(&last_artifact)
        .args(&["-s", "ftFunction"])
        .arg("-t")
        .arg(dep_files::get(symbols)?)
        .arg("-l")
        .arg(dep_files::get("melee.link")?)
        .args(&["-q", "-ow", "-w", "-c", "-dat"])
        .arg(&dat_path)
        .status()
        .unwrap();

    if output.success() {
        Ok(dat_path)
    } else {
        Err(Error::ExitStatus(output.code().unwrap())) 
    }
}

fn to_file_name(name: &str) -> Result<&str, Error> {
    if SYMBOLS_PROPER_NAMES.contains(&name) {
        Ok(SYMBOLS_ALLOW_LIST[SYMBOLS_PROPER_NAMES.iter().position(|x| *x == name).unwrap()])
    } else if SYMBOLS_ALLOW_LIST.contains(&name) {
        Ok(name)
    } else {
        Err(Error::InvalidSymbolName)
    }
}

pub const SYMBOLS_ALLOW_LIST: &[&str] = &[
    "cssFunction.txt", // css
    "evFunction.txt", // event
    "ftFunction.txt", // fighter
    "grFunction.txt", // stage
    "itFunction.txt", // item
    "kbFunctoin.txt", // kirby
    "mjFunction.txt", // major_scene
    "mnFunction.txt", // minor_scene
    "tmFunction.txt", // tournament
];

pub const SYMBOLS_PROPER_NAMES: &[&str] = &[
    "css",
    "event",
    "fighter",
    "stage",
    "item",
    "kirby",
    "major_scene",
    "minor_scene",
    "tournament",
];

const TARGET_NAME: &str = "powerpc-unknown-linux-gnu";

fn ensure_nightly_ppc_installed() {
    // rustup +nightly toolchain list
    let nightly_installed = 
            String::from_utf8(
                Command::new("rustup")
                    .args(&["toolchain", "list"])
                    .output()
                    .unwrap()
                    .stdout
            )   
            .unwrap()
            .split('\n')
            .any(|toolchain| toolchain.trim().starts_with("nightly-"));

    if !nightly_installed {
        // rustup toolchain install nightly
        Command::new("rustup")
            .args(&["toolchain", "install", "nightly"])
            .status()
            .unwrap();
    }

    // rustup +nightly target list --installed
    let ppc_installed = 
            String::from_utf8(
                Command::new("rustup")
                    .args(&["target", "list", "--toolchain", "nightly", "--installed"])
                    .output()
                    .unwrap()
                    .stdout
            )
            .unwrap()
            .split('\n')
            .any(|target| target.trim() == TARGET_NAME);

    if !ppc_installed {
        // rustup +nightly target install powerpc-unknown-linux-gnu
        Command::new("rustup")
            .args(&["target", "install", "--toolchain", "nightly", TARGET_NAME])
            .status()
            .unwrap();
    }
}
