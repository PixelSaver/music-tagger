use m3u8_rs;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::error::*;
use crate::core::{models, models::{Track, TrackLocation}};

pub fn load_m3u8(path: &Path) -> Result<models::Playlist> {
    let file = fs::read(path)?;
    let playlist = m3u8_rs::parse_playlist_res(&file)
        .map_err(|e| MusicTaggerError::InvalidPlaylist(format!("{:?}", e)))?;
    match playlist {
        m3u8_rs::Playlist::MediaPlaylist(media) => {
            let model_playlist = models::Playlist::new();
            for segment in media.segments {
                let mut track_loc = TrackLocation::new(
                    Track::new(),
                    PathBuf::from(segment.uri),
                );
                track_loc.track.track_title = segment.title.unwrap_or_default();
                // track.duration= segment.duration as f32;
            }
            return Ok(model_playlist)
        },
        m3u8_rs::Playlist::MasterPlaylist(_) => {
            return Err(MusicTaggerError::InvalidPlaylist("Playlist is a master playlist. Master playlists are not yet supported.".into()));
        },
    }
}
pub fn save_m3u8(path: &Path, playlist: &models::Playlist) -> Result<()> {
    let mut out = String::new();
    // Required header for m3u and m3u8 files
    out.push_str("#EXTM3U\n");

    for track_loc in &playlist.track_locations {
        let track = &track_loc.track;
        // TODO Set the duration in the playlist
        let duration = track_loc.track.duration.div_ceil(1000);
        let title = if track.track_title.is_empty() { "Unknown" } else { &track.track_title };
        out.push_str(&format!("#EXTINF:{:?}, {}\n", duration, title));

        out.push_str(&format!(
            "{}\n",
            track_loc.path.to_string_lossy(),
        ))
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    };

    let mut file = fs::File::create(path)?;
    file.write_all(out.as_bytes())?;
    Ok(())
}