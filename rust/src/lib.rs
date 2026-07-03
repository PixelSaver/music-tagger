pub mod error;
pub mod app;
pub mod core;
pub mod media;
pub mod library;
// pub mod listener;
pub mod playlists;
// pub mod util;

use std::path::Path;

use godot::prelude::*;
use godot::classes::Node;

struct MusicTaggerGDExtension;
use crate::core::models::*;
use std::path::PathBuf;

#[gdextension]
unsafe impl ExtensionLibrary for MusicTaggerGDExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
struct MusicTagger {
    pub library: Option<Library>,
    pub playlist_directory: PathBuf,
    pub music_directories: Vec<PathBuf>,
    pub cache_directory: PathBuf,
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl INode for MusicTagger {
    fn init(base: Base<Node>) -> Self {
        Self {
            library: None,
            playlist_directory: PathBuf::new(),
            music_directories: Vec::new(),
            cache_directory: PathBuf::new(),
            base,
        }
    }
    fn process(&mut self, _delta: f64) {
        
    }
}
#[godot_api]
impl MusicTagger {
    #[func]
    pub fn scan_directory(&mut self, directory: String) -> String {
        let library = match crate::library::cache::load_library(Path::new(&directory)) {
            Ok(lib) => lib,
            Err(e) => {
                return e.to_string();
            },
        };
        self.library = library;
        if self.library.is_none() {
            return "No library loaded / found.".into();
        }
        
        return "".into();
    }
}