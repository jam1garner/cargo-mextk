use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("an io error occurred: {0:?}")]
    Io(#[from] io::Error),

    #[error("cargo exited with a status of {0}")]
    ExitStatus(i32),

    #[error("the directory being created already exists")]
    DirAlreadyExists,

    #[error("cargo produced an unparsable stream of data")]
    FailParseCargoStream,

    #[error("no build artifact was produced by cargo")]
    NoBuildArtifact,
}
