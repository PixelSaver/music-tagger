use std::{path::Path, fs::File};
use serde::{de::DeserializeOwned, Serialize};
use crate::core::models::*;
use crate::error::*;

pub fn load_library(cache_path: &Path) -> Result<Option<Library>> {
    log::debug!("Trying cache at path: {:?}", cache_path);
    let lib: Option<Library> = load_json(cache_path)?;
    match lib {
        Some(lib) => Ok(Some(lib)),
        None => {
            log::debug!("Failed to find cache.");
            return Ok(None);
        }
    }
    // return Ok(scanner::walk_dir(&PathBuf::from(backup_path))?)
}
pub fn save_library(cache_path: &Path, library: &Library) -> Result<()> {
    save_json(cache_path, library)
}

pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}
pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    let file = File::create(path)?;
    serde_json::to_writer(file, data)?;
    Ok(())
}