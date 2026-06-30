use std::path::PathBuf;
use crate::error::*;
use crate::core::models::Track;
use walkdir::WalkDir;

pub fn walk_dir(dir: PathBuf) -> Result<Vec<Track>> {
    let walkdir = WalkDir::new(dir);
    
    for entry in walkdir.follow_links(true).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
    return Ok(Vec::new());
}