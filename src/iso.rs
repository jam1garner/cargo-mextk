use crate::{paths::{self, PathExt}, Error};

use memmap::Mmap;
use rayon::prelude::*;
use gc_gcm::{GcmFile, DirEntry, File, GcmError};
use humansize::{FileSize, file_size_opts as options};
use cli_table::{print_stdout, Table, Title};
use cli_table::format::{Justify, Border, Separator, HorizontalLine, VerticalLine};

use std::fs;
use std::path::{Path, PathBuf};

pub fn add(iso_path: &Path, output: bool) -> Result<(), Error> {
    let iso = match GcmFile::open(iso_path) {
        Ok(iso) => iso,
        Err(GcmError::IoError(err)) => Err(err)?,
        Err(GcmError::ParseError(_)) => return Err(Error::InvalidGcm),
    };

    let id = match std::str::from_utf8(&iso.game_id.0[..]) {
        Ok(id) => format!("{}_v{}", id, iso.revision),
        Err(_) => panic!("game id isn't valid UTF-8")
    };

    let game_dir = paths::iso_dir().push_join(id).ensure_exists();
    let extracted_path = game_dir.join("extracted").ensure_exists();
    
    if output {
        println!("Copying ISO...");
    }

    fs::copy(iso_path, game_dir.join("game.iso"))?;

    if output {
        println!("Extracting ISO...");
    }

    extract(iso, &iso_path, &extracted_path, false);

    if output {
        println!("Success!");
    }

    Ok(())
}

pub fn remove(iso_id: &str) -> Result<(), Error> {
    let iso_folder = paths::iso_dir().push_join(iso_id);

    if iso_folder.exists() {
        fs::remove_dir_all(iso_folder)?;

        Ok(())
    } else {
        Err(Error::NoSuchIso)
    }
}

#[derive(Table)]
pub struct EntryDisplay<'a> {
    #[table(name = "Image ID", justify = "Justify::Left")]
    id: &'a str,

    #[table(name = "Name", justify = "Justify::Left")]
    name: &'a str,

    #[table(name = "Size", justify = "Justify::Right")]
    size: &'a str,
    
    #[table(name = "Path")]
    path: &'a str,
}

pub struct Entry {
    pub id: String,
    pub size: String,
    pub path: PathBuf,
    pub path_display: String,
    pub name: String,
}

fn get_iso_name(path: &Path) -> String {
    GcmFile::open(path)
        .map(|iso| iso.internal_name)
        .unwrap_or_else(|_| "???".to_string())
}

pub fn list() -> Result<Vec<Entry>, Error> {
    let iso_dir = paths::iso_dir();
    Ok(
        fs::read_dir(&iso_dir)?
            .filter_map(|iso| {
                iso.ok().map(|iso| {
                    let id = iso.file_name().to_string_lossy().into_owned();
                    let size = if let Ok(meta) = fs::metadata(iso.path().push_join("game.iso")) {
                        meta.len()
                            .file_size(options::BINARY)
                            .unwrap()
                    } else {
                        "??? MiB".to_owned()
                    };

                    let path = iso_dir.join(iso.path());
                    let path_display = path.display().to_string();

                    let name = get_iso_name(&path.join("game.iso"));

                    Entry { id, size, path, path_display, name }
                })
            })
            .collect()
    )
}

pub fn display_list(listing: Vec<Entry>) {
    let listing = listing.iter().map(Entry::display);
    print_stdout(
        listing
            .table()
            .title(EntryDisplay::title())
            .border(
                Border::builder()
                    .top(HorizontalLine::new(' ', ' ', ' ', ' '))
                    .bottom(HorizontalLine::new(' ', ' ', ' ', ' '))
                    .left(VerticalLine::new(' '))
                    .right(VerticalLine::new(' '))
                    .build()
            )
            .separator(
                Separator::builder()
                    .column(None)
                    .row(None)
                    .title(Some(HorizontalLine::new('-', '-', '-', '-')))
                    .build()
            )
    ).unwrap();
}

impl Entry {
    pub fn display(&self) -> EntryDisplay {
        EntryDisplay {
            id: &self.id,
            size: &self.size,
            path: &self.path_display,
            name: &self.name,
        }
    }
}

fn extract_entry<'a>(entry: DirEntry<'a>, path: &Path, files: &mut Vec<(PathBuf, File)>) {
    if let Some(file_data) = entry.as_file() {
        files.push((path.join(entry.entry_name()), file_data));
    } else if let Some(entries) = entry.iter_dir() {
        let dir_path = path.join(entry.entry_name());
        let _ = fs::create_dir_all(&dir_path);
        for child in entries {
            extract_entry(child, &dir_path, files)
        }
    }
}

fn extract(iso: GcmFile, path: &Path, to: &Path, single_thread: bool) {
    let file = std::fs::File::open(&path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };

    let mut files = Vec::new();
    for entry in iso.filesystem.iter_root() {
        extract_entry(entry, to, &mut files)
    }

    let iso = &mmap[..];

    let extract_file = |(path, file): &(PathBuf, File)| {
        let start = file.offset as usize;
        let end = start + (file.size as usize);
        let file = &iso[start..end];
        
        if let Err(err) = fs::write(path, file) {
            println!("Path: {}", path.display());
            println!("Error: {:?}", err);
            println!();
        }
    };

    if single_thread {
        files.iter().for_each(extract_file);
    } else {
        files.par_iter().for_each(extract_file);
    }
}
