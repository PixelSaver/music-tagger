use crate::core::models::*;
use crate::error::*;
use serde::{Serialize, de::DeserializeOwned};
use std::{fs::File, path::Path};

pub fn load_library(cache_path: &Path) -> Result<Library> {
    //TODO Don't cache art
    log::debug!("Trying cache at path: {:?}", cache_path);
    let lib: Library = load_bincode(cache_path)?;
    log::debug!("Cache loaded successfully!");
    Ok(lib)
    // return Ok(scanner::walk_dir(&PathBuf::from(backup_path))?)
}
pub fn save_library(cache_path: &Path, library: &Library) -> Result<()> {
    save_bincode(cache_path, library)
}

// pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
//     let file = File::open(path)?;
//     Ok(serde_json::from_reader(file)?)
// }
// pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<()> {
//     let file = File::create(path)?;
//     serde_json::to_writer(file, data)?;
//     Ok(())
// }
pub fn load_bincode<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = File::open(path)?;
    log::debug!("Loading bincode from: {:?}", path);
    match bincode::deserialize_from(file) {
        Ok(data) => {
            log::debug!("Loaded bincode!");
            Ok(data)
        }
        Err(e) => {
            log::debug!("Failed to load bincode: {:?}", e);
            Err(e.into())
        }
    }
}
pub fn save_bincode<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    // Make sure the parent folder exists (if there is a parent path)
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?; // creates recursively if missing
    }
    let file = File::create(path)?;
    bincode::serialize_into(file, data)?;
    Ok(())
}
