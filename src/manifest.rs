use crate::Error;

use std::{fs, env};
use std::path::Path;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub dat: Option<String>,
    pub game_id: Option<String>,
    pub symbols: Option<String>,
}

impl Manifest {
    pub fn from_current_directory() -> Result<Self, Error> {
        let mut cwd = env::current_dir()?;

        Self::from_path(&cwd)
            .or_else(move |err| {
                while cwd.pop() {
                    if let manifest @ Ok(_) = Self::from_path(&cwd) {
                        return manifest;
                    }
                }

                Err(err)
            })
    }

    pub fn from_path<P>(path: P) -> Result<Self, Error>
        where P: AsRef<Path>,
    {
        let path = path.as_ref().join("Mextk.toml");

        let contents = fs::read_to_string(path).or(Err(Error::NoMexToml))?;
        
        toml::from_str(&contents).map_err(Error::InvalidToml)
    }
}
