pub mod error;
pub mod app;
pub mod core;
pub mod media;
pub mod library;
// pub mod listener;
pub mod playlists;
// pub mod util;
// 
pub mod godot_log;

use std::path::{self, Path};

use godot::prelude::*;
use godot::classes::Node;

struct MusicTaggerGDExtension;
use crate::error::*;
use crate::core::models::*;
use crate::godot_log::event::{EventReporter, MusicTaggerEvent};
use std::path::PathBuf;

#[gdextension]
unsafe impl ExtensionLibrary for MusicTaggerGDExtension {}

#[derive(GodotClass)]
#[class(base = Node)]
struct MusicTaggerNode {
    pub library: Option<Library>,
    pub playlist_directory: PathBuf,
    pub music_directories: Vec<PathBuf>,
    pub cache_directory: PathBuf,
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl INode for MusicTaggerNode {
    fn init(base: Base<Node>) -> Self {
        crate::godot_log::godot_log::init_logger();
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
impl MusicTaggerNode {
    #[signal]
    fn scan_progress(path: String);
    #[signal]
    fn error(message: String);
    #[signal]
    fn track_found(title: String);
    
    #[func]
    pub fn scan_directory(&mut self, directory: String) -> String {
        let library = match crate::library::scanner::walk_dir(Path::new(&directory), self) {
            Ok(lib) => lib,
            Err(e) => {
                return e.to_string();
            },
        };
        self.library = Some(library);
        if self.library.is_none() {
            return "No library loaded / found.".into();
        }
        
        return "".into();
    }
}
impl EventReporter for MusicTaggerNode {
    fn emit(&mut self, event: MusicTaggerEvent) {
        match event {
            MusicTaggerEvent::Scanning(path) => {
                godot_print!("Scanning: {}", path.display());
                self.base_mut().emit_signal(
                    "scan_progress",
                    &[path.display().to_string().to_variant()]
                );
            },
            MusicTaggerEvent::TrackFound(track) => {
                godot_print!("Track found: {}", track);
                self.base_mut().emit_signal(
                    "track_found",
                    &[track.to_variant()]
                );
            },
            MusicTaggerEvent::Error(error) => {
                godot_error!("Error: {}", error);
                self.base_mut().emit_signal(
                    "error",
                    &[error.to_variant()]
                );
            },
            _ => {
                
            }
        }
    }
}