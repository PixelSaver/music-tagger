use std::{path::{Path, PathBuf}, fs::File};
use crate::core::models::*;
use crate::error::*;
use crate::library::scanner;

pub fn load_cache(cache_path: &Path) -> Result<Option<Library>> {
    for entry in cache_path.read_dir()? {
        let entry = entry?;

        let path = entry.path();

        if path.is_file() {
            let file = File::open(&path)?;

            let library: Library = serde_json::from_reader(file)?;
            return Ok(Some(library))
        }
    }
    log::debug!("Failed to find cache.");
    return Ok(None);
    // return Ok(scanner::walk_dir(&PathBuf::from(backup_path))?)
}
pub fn store_cache(cache_path: &Path, library: &Library) -> Result<()> {
    let cache_file = if cache_path.is_dir() {
        File::create(&PathBuf::from(cache_path).join("library.json"))?
    } else {
        File::create(cache_path)?
    };
    serde_json::to_writer(cache_file, library)?;
    Ok(())
}