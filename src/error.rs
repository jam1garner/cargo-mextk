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

    #[error("provided ISO is not in the GCM format")]
    InvalidGcm,

    #[error("no such iso id exists. Add an iso with `cargo mextk add-iso [iso]`")]
    NoSuchIso,

    #[error("MexTK installation could not be found. Install it to {0}.")]
    NoMextkInstalled(String),

    #[error("MexTK .NET core installation found, but `dotnet` has not been added to path.")]
    NoDotNet,

    #[error("A network error occurred while attempting to download required files")]
    NetworkError,
}
