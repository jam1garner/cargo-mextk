use std::path::PathBuf;
use std::fs::File;

use crate::Error;
use crate::paths::{PathExt, mextk_deps_dir};

fn file_url(file: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/UnclePunch/Training-Mode/master/MexTK/{}",
        file
    )
}

fn download(path: PathBuf, file: &str) -> Result<PathBuf, Error> {
    let response = ureq::get(&file_url(file)).call().or(Err(Error::NetworkError))?;

    let mut file = File::create(&path)?;

    std::io::copy(&mut response.into_reader(), &mut file)?;

    Ok(path)
}

pub fn get(file: &str) -> Result<PathBuf, Error> {
    let path = mextk_deps_dir().push_join(file);

    if !path.exists() {
        download(path, file)
    } else {
        Ok(path)
    }
}
