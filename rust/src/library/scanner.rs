use std::path::Path;
use crate::error::*;
use crate::godot_log::event::{EventReporter, MusicTaggerEvent};
use crate::media::media;
use crate::core::models::{Library, TrackLocation};
use walkdir::WalkDir;

pub fn walk_dir(dir: &Path, reporter: &mut dyn EventReporter) -> Result<Library> {
    let parent = dir.parent().unwrap_or(dir);
    let walkdir = WalkDir::new(dir);
    let mut out = Vec::new();
    for entry in walkdir
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file()) {

            reporter.emit(MusicTaggerEvent::Scanning(entry.path()));
            let mut file = std::fs::File::open(entry.path())?;
            match media::read_track_from_file(&mut file) {
                Ok((track, lofty_tagged_file)) => {
                    log::debug!("Track: {:?}", track);
                    reporter.emit(MusicTaggerEvent::TrackFound(&track.track_title));
                    let relative_path = entry.path()
                        .strip_prefix(parent)?.to_path_buf();
                    out.push(TrackLocation {
                        track,
                        path: (relative_path).into(),
                        lofty_tagged_file: Some(lofty_tagged_file),
                    });
                }
                Err(e) => {
                    log::debug!("Error when reading track: {:?}", e);
                    reporter.emit(MusicTaggerEvent::Error(&e.to_string()));
                    continue;
                }
            }
        }
        return Ok(Library{
            tracks: out,
        });
}