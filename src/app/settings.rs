use crate::core::models::Library;
use std::path::PathBuf;


pub struct Settings {
    pub library: Option<Library>,
    pub playlist_directory: PathBuf,
    pub music_directories: Vec<PathBuf>,
    pub cache_directory: PathBuf,
}
impl Settings {
    pub fn new() -> Self {
        Self {
            library: None,
            playlist_directory: PathBuf::from("."),
            music_directories: vec![PathBuf::from(".")],
            cache_directory: PathBuf::from("./.cache/"),
        }
    }
}