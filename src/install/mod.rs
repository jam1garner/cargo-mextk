use crate::paths::extracted_dat_path;
use crate::manifest::Manifest;
use crate::{build, iso, Error};
use std::fs;

mod patches;
mod additions;
//mod patch_dol;

pub fn install(restore: bool) -> Result<(), Error> {
    let toml = Manifest::from_current_directory()?;
    let dat_name = toml.dat.ok_or(Error::NoDatName)?;
    let id = toml.game_id.as_deref().unwrap_or("GALE01_v2");

    if restore {
        // Restore iso before installing mods
        iso::restore(id, false)?;

        // Install m-ex files
        patches::apply(id)?;
        additions::add(id)?;
    }

    let path = build(false)?;
    let dat_path = extracted_dat_path(id, &dat_name);

    fs::copy(path, dat_path)?;

    Ok(())
}
