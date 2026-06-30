use std::path::PathBuf;
use crate::error::*;
use crate::media::media;
use crate::core::models::{Library, TrackLocation};
use walkdir::WalkDir;

pub fn walk_dir(dir: PathBuf) -> Result<Library> {
    let walkdir = WalkDir::new(dir);
    let mut out = Vec::new();
    for entry in walkdir.follow_links(true).into_iter().filter_map(|e| e.ok()) {
        let mut file = std::fs::File::open(entry.path())?;
        match media::read_track_from_file(&mut file) {
            Ok(track) => {
                out.push(TrackLocation {
                    track,
                    path: PathBuf::from(entry.path()),
                });
            }
            Err(_) => {
                continue;
            }
        }
    }
    return Ok(Library{
        tracks: out,
    });
}