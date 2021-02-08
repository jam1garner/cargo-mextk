use dirs_next::home_dir;

use std::path::{Path, PathBuf};
use std::fs;

pub fn mextk_dir() -> PathBuf {
    home_dir()
        .unwrap()
        .push_join(".mextk")
        .ensure_exists()
}

pub fn iso_dir() -> PathBuf {
    mextk_dir()
        .push_join("iso")
        .ensure_exists()
}

pub fn dir_from_id(id: &str) -> PathBuf {
    iso_dir().push_join(id)
}

pub(crate) trait PathExt {
    fn ensure_exists(self) -> Self;
    fn push_join<P: AsRef<Path>>(self, join: P) -> Self;
}

impl PathExt for PathBuf {
    fn ensure_exists(self) -> Self {
        if !self.exists() {
            fs::create_dir_all(&self).unwrap();
        }

        self
    }

    fn push_join<P: AsRef<Path>>(mut self, join: P) -> Self {
        self.push(join);

        self
    }
}
