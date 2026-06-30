use crate::core::models::Library;
use std::path::PathBuf;


pub struct Settings {
    library: Option<Library>,
    playlist_directory: PathBuf,
    music_directories: Vec<PathBuf>,
    cache_directory: PathBuf,
}