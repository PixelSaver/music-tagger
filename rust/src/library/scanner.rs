use crate::core::models::{Library, TrackLocation};
use crate::error::*;
use crate::godot_log::event::MusicTaggerEvent;
use crate::media::media;
use flume::Sender;
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, atomic::AtomicBool};
use std::sync::atomic::{AtomicUsize, Ordering};
use walkdir::WalkDir;

pub fn walk_dir(dir: &Path, sender: &Sender<MusicTaggerEvent>, cancel_scan: Arc<AtomicBool>) -> Result<Library> {
    let paths: Vec<PathBuf> = WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let total = paths.len() as i32;
    let _ = sender.send(MusicTaggerEvent::ProgressStarted(total));

    let processed = Arc::new(AtomicUsize::new(0));
    let target_ticks = 200;
    let tick_every: i32 = (total / target_ticks).max(1);

    let tracks: Vec<TrackLocation> = paths
        .par_iter()
        .filter_map(|path| {
            if cancel_scan.load(Ordering::Relaxed) {
                return None;
            }
            let mut file = File::open(path).ok()?;
            if cancel_scan.load(Ordering::Relaxed) {
                return None;
            }

            let result = match media::read_track_from_file(&mut file) {
                Ok((track, lofty_tagged_file)) => {
                    let _ = sender.send(MusicTaggerEvent::TrackFound(track.track_title.clone()));
                    Some(TrackLocation {
                        track,
                        path: path.clone(),
                        lofty_tagged_file: Some(lofty_tagged_file),
                    })
                }
                Err(_e) => {
                    // Skip unreadable or errored files
                    return None;
                }
            };

            // Ticking progress bar
            let prev = processed.fetch_add(1, Ordering::Relaxed) as i32;
            let done = prev + 1;
            if done == total || done % tick_every == 0 {
                let _ = sender.send(MusicTaggerEvent::ProgressTick(done));
            }
            result
        })
        .collect();
    
    if cancel_scan.load(Ordering::Relaxed) {
        return Err(MusicTaggerError::Cancelled);
    }
    let _ = sender.send(MusicTaggerEvent::ProgressTick(total));
    Ok(Library { tracks })
}
