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

use std::path::Path;

use godot::prelude::*;
use godot::classes::{Image, Node};

struct MusicTaggerGDExtension;
// use crate::error::*;
use crate::core::models::*;
use crate::godot_log::event::{EventReporter, MusicTaggerEvent};

#[gdextension]
unsafe impl ExtensionLibrary for MusicTaggerGDExtension {}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GodotTrack {
    #[var]
    pub track_title: GString,
    #[var]
    pub composer: GString,
    #[var]
    pub isrc: GString,
    #[var]
    pub track_artist: GString,
    #[var]
    pub track_artists: GString,
    #[var]
    pub cover_art: Option<Gd<Image>>,
    #[var]
    pub copyright_message: GString,
    #[var]
    pub description: GString,
    #[var]
    pub publisher: GString,
    // pub album: Option<Album>,
    #[var]
    pub genre: GString,
    #[var]
    pub duration: i32,
    #[var]
    pub custom_tags: Array<GString>,
    #[base]
    base: Base<RefCounted>,
}
#[godot_api]
impl IRefCounted for GodotTrack {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            track_title: GString::new(),
            composer: GString::new(),
            isrc: GString::new(),
            track_artist: GString::new(),
            track_artists: GString::new(),
            cover_art: None,
            copyright_message: GString::new(),
            description: GString::new(),
            publisher: GString::new(),
            genre: GString::new(),
            duration: 0,
            custom_tags: Array::<GString>::new(),
            base,
        }
    }
}
#[godot_api]
impl GodotTrack {
    fn from_track(track: Track, base: Base<RefCounted>) -> Self {
        let cover_art = if let Some(cover_art) = track.cover_art {
            let mut image = Image::new_gd();
            
            match cover_art.mime_type.as_deref() {
                Some("image/png") => {
                    image.load_png_from_buffer(&PackedByteArray::from(cover_art.data));
                    Some(image)
                }
                Some("image/jpeg") | Some("image/jpg") => {
                    image.load_jpg_from_buffer(&PackedByteArray::from(cover_art.data));
                    Some(image)
                }
                _ => None,
            }
        } else { None };
        Self {
            track_title: GString::from(track.track_title.as_str()),
            composer: GString::from(track.composer.as_str()),
            isrc: GString::from(track.isrc.as_str()),
            track_artist: GString::from(track.track_artist.as_str()),
            track_artists: GString::from(track.track_artists.as_str()),
            cover_art,
            copyright_message: GString::from(track.copyright_message.as_str()),
            description: GString::from(track.description.as_str()),
            publisher: GString::from(track.publisher.as_str()),
            genre: GString::from(track.genre.as_str()),
            duration: track.duration as i32,
            custom_tags: Array::<GString>::new(),
            base,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node)]
struct MusicTaggerNode {
    pub library: Option<Library>,
    #[export]
    pub playlist_directory: GString,
    #[export]
    pub music_directories: Array<GString>,
    #[export]
    pub cache_directory: GString,
    #[base]
    base: Base<Node>,
}
#[godot_api]
impl INode for MusicTaggerNode {
    fn init(base: Base<Node>) -> Self {
        crate::godot_log::godot_log::init_logger();
        Self {
            library: None,
            playlist_directory: GString::new(),
            music_directories: Array::<GString>::new(),
            cache_directory: GString::new(),
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
    pub fn get_all_tracks(&self) -> Array<Gd<GodotTrack>> {
        let mut tracks = Array::<Gd<GodotTrack>>::new();
        for track in self.library.as_ref().map(|lib| lib.tracks.iter()).unwrap_or_default() {
            let gd_track = Gd::from_init_fn(|base| GodotTrack::from_track(track.track.clone(), base));
            tracks.push(&gd_track);
        }
        tracks
    }

    #[func]
    pub fn get_all_genres(&self) -> Array<GString> {
        let mut out = Array::<GString>::new();
        let library = self.library.as_ref();
        if library.is_none() { return out; }
        let library = library.unwrap();
        library.tracks.iter().for_each(|track| {
            if out.find(&track.track.genre, 0.into()).is_none() {
                out.push(&track.track.genre);
            };
        });
        
        out
    }
    
    #[func]
    pub fn get_track_count(&self) -> i32 {
        self.library
            .as_ref()
            .map(|lib| lib.tracks.len() as i32)
            .unwrap_or(0)
    }
    
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
            // _ => {
                
            // }
        }
    }
}