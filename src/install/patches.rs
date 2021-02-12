use std::fs;

use crate::Error;
use crate::paths::{dir_from_id, PathExt};

use include_dir::{Dir, File, include_dir};

const PATCHES: Dir = include_dir!("src/mex-patches");

pub fn apply(id: &str) -> Result<(), Error> {
    let dir = dir_from_id(id).push_join("extracted");

    let files = get_files(&PATCHES);
    
    for patch_file in files {
        let path = dir.join(patch_file.path);
        //println!("patching: {}", path.display());

        let data = fs::read(&path).unwrap();
        
        let patched = xdelta3::decode(patch_file.contents(), &data).ok_or(Error::PatchFailed)?;

        fs::write(path, patched).unwrap();
    }

    Ok(())
}

fn get_files<'a>(dir: &'a Dir) -> Vec<File<'a>> {
    if dir.dirs().len() == 0 {
        dir.files.to_owned()
    } else {
        dir.dirs()
            .iter()
            .map(|dir| get_files(dir).into_iter())
            .flatten()
            .chain(dir.files().iter().map(|x| *x))
            .collect()
    }
}
