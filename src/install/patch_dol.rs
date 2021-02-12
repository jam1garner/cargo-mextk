use std::path::{Path, PathBuf};
use std::io::{self, BufReader};
use std::fs::{self, File};

use bsdiff::patch::patch as bsdiff_patch;

use crate::paths::{mextk_deps_dir, PathExt};
use crate::Error;

const PATCH_URL: &str = "https://github.com/akaneia/mexTool/raw/master/mexTool/lib/dol.patch";

fn download_patch(path: PathBuf) -> Result<PathBuf, Error> {
    let mut patch = ureq::get(PATCH_URL)
        .call()
        .or(Err(Error::NetworkError))?
        .into_reader();

    io::copy(
        &mut patch,
        &mut File::create(&path)?
    ).unwrap();

    Ok(path)
}

fn get_patch_path() -> Result<PathBuf, Error> {
    let path = mextk_deps_dir().push_join("dol.patch");

    if path.exists() {
        Ok(path)
    } else {
        download_patch(path)
    }
}

pub fn apply(dol: &Path) -> Result<(), Error> {
    let mut patch_file = BufReader::new(File::open(get_patch_path()?).unwrap());

    let old_dol = fs::read(dol).unwrap();
    let mut new_dol = vec![0; old_dol.len()];

    bsdiff_patch(&old_dol[..], &mut patch_file, &mut new_dol)
        .or(Err(Error::DolPatchFail))?;

    fs::write(dol, new_dol)
        .or(Err(Error::DolPatchFail))
}
