use std::path::{Path, PathBuf};
use crate::error::*;
use crate::media::media;
use crate::core::models::{Library, TrackLocation};
use walkdir::WalkDir;

pub fn walk_dir(dir: &Path) -> Result<Library> {
    let parent = dir.parent().unwrap_or(dir);
    let walkdir = WalkDir::new(dir);
    let mut out = Vec::new();
    for entry in walkdir
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) {
        let mut file = std::fs::File::open(entry.path())?;
        println!("Scanning: {:?}", entry.path());
        match media::read_track_from_file(&mut file) {
            Ok(track) => {
                log::debug!("Track: {:?}", track);
                let relative_path = entry.path()
                    .strip_prefix(parent)?.to_path_buf();
                out.push(TrackLocation {
                    track,
                    path: (relative_path).into(),
                });
            }
            Err(e) => {
                log::debug!("Error when reading track: {:?}", e);
                continue;
            }
        }
    }
    return Ok(Library{
        tracks: out,
    });
}