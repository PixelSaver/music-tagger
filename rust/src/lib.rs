pub mod app;
pub mod core;
pub mod error;
pub mod library;
pub mod media;
// pub mod listener;
pub mod playlists;
// pub mod util;
//
pub mod godot_log;

use std::path::{Path, PathBuf};

use godot::classes::{Image, Node};
use godot::prelude::*;
use strsim::jaro_winkler;

struct MusicTaggerGDExtension;
// use crate::error::*;
use crate::core::models::*;
use crate::godot_log::event::MusicTaggerEvent;
use crate::library::search::search_tracks;
use crate::media::convert::get_cover_art;

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
    pub genres: Array<GString>,
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
            genres: Array::<GString>::new(),
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
        } else {
            None
        };
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
            genres: track
                .genre
                .iter()
                .map(|genre| GString::from(genre.as_str()))
                .collect::<Array<GString>>(),
            duration: track.duration as i32,
            custom_tags: track
                .custom_tags
                .iter()
                .map(|tag| GString::from(tag.value.as_str()))
                .collect::<Array<GString>>(),
            base,
        }
    }
}

impl TrackPicture {
    pub fn to_gd_image(self) -> Option<Gd<Image>> {
        let mut image = Image::new_gd();
        let bytes = PackedByteArray::from(self.data);

        match self.mime_type.as_deref() {
            Some("image/png") => image.load_png_from_buffer(&bytes),
            Some("image/jpeg") | Some("image/jpg") => image.load_jpg_from_buffer(&bytes),
            _ => return None,
        };

        Some(image)
    }
}

struct CoverRequest {
    isrc: String,
    path: PathBuf,
}

#[derive(GodotClass)]
#[class(base = Node)]
struct MusicTaggerNode {
    cover_request_tx: flume::Sender<CoverRequest>,
    event_tx: flume::Sender<MusicTaggerEvent>,
    receiver: flume::Receiver<MusicTaggerEvent>,

    pub library: Option<Library>,
    pub godot_tracks: Array<Gd<GodotTrack>>,
    #[export]
    pub searched_track_idxs: Array<i32>,
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
        let (request_tx, request_rx) = flume::unbounded::<CoverRequest>();
        let (event_tx, event_rx) = flume::unbounded::<MusicTaggerEvent>();
        let cover_event_tx = event_tx.clone();
        std::thread::spawn(move || {
            while let Ok(mut request) = request_rx.recv() {
                while let Ok(newer) = request_rx.try_recv() {
                    // Replace old requests with the newer one, only need one request running 
                    request = newer;
                }
                let cover = get_cover_art(request.path).ok();
                let _ = cover_event_tx.send(MusicTaggerEvent::LoadedCoverArt(request.isrc, cover));
            }
        });
        Self {
            cover_request_tx: request_tx,
            event_tx,
            searched_track_idxs: Array::new(),
            receiver: event_rx,
            library: None,
            godot_tracks: Array::new(),
            playlist_directory: GString::new(),
            music_directories: Array::<GString>::new(),
            cache_directory: GString::new(),
            base,
        }
    }
    fn process(&mut self, _delta: f64) {
        let mut events = Vec::new();
        while let Ok(event) = self.receiver.try_recv() {
            events.push(event);
        }
        for event in events {
            match event {
                MusicTaggerEvent::Scanning(path) => {
                    self.base_mut()
                        .emit_signal("scan_progress", &[path.display().to_string().to_variant()]);
                }
                MusicTaggerEvent::TrackFound(title) => {
                    self.base_mut()
                        .emit_signal("track_found", &[title.to_variant()]);
                }
                MusicTaggerEvent::Finished(Ok(library)) => {
                    let mut arr: Array<Gd<GodotTrack>> = Array::new();
                    for track in &library.tracks {
                        arr.push(&Gd::from_init_fn(|base| {
                            GodotTrack::from_track(track.track.clone(), base)
                        }));
                    }
                    let cache_dir = self.cache_directory.to_string();
                    let path = Path::new(&cache_dir);
                    let canonical_path = if let Ok(canonical) = path.canonicalize() {
                        canonical
                    } else {
                        PathBuf::from(path)
                    };
                    log::debug!("Saving library cache to {:?}", canonical_path);
                    if let Err(e) = crate::library::cache::save_library(path, &library) {
                        self.base_mut()
                            .emit_signal("error", &[e.to_string().to_variant()]);
                    }
                    self.library = Some(library);
                    self.godot_tracks = self.get_all_tracks();
                    self.base_mut().emit_signal("library_scanned", &[]);
                }
                MusicTaggerEvent::Finished(Err(e)) => {
                    self.base_mut()
                        .emit_signal("error", &[e.to_string().to_variant()]);
                }
                MusicTaggerEvent::LoadedCoverArt(isrc, Some(cover)) => {
                    if let Some(library) = &mut self.library {
                        if let Some(track_loc) = library.tracks.iter_mut().find(|t| t.track.isrc == isrc) {
                            track_loc.track.cover_art = Some(cover.clone());
                        }
                    }
                    let img = cover.to_gd_image();
                    self.base_mut().emit_signal(
                        "loaded_cover_art",
                        &[Variant::from(isrc), Variant::from(img)],
                    );
                }
                MusicTaggerEvent::LoadedCoverArt(isrc, None) => {
                    self.base_mut().emit_signal(
                        "error",
                        &[Variant::from(format!(
                            "Failed to fetch cover art data from {}",
                            isrc
                        ))],
                    );
                }
                MusicTaggerEvent::Error(msg) => {
                    self.base_mut().emit_signal("error", &[msg.to_variant()]);
                }
            }
        }
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
    #[signal]
    fn library_scanned();
    #[signal]
    fn loaded_cover_art(isrc: String, cover_art: Option<Gd<Image>>);

    #[func]
    pub fn get_all_tracks(&self) -> Array<Gd<GodotTrack>> {
        let mut tracks = Array::<Gd<GodotTrack>>::new();
        for track in self
            .library
            .as_ref()
            .map(|lib| lib.tracks.iter())
            .unwrap_or_default()
        {
            let gd_track =
                Gd::from_init_fn(|base| GodotTrack::from_track(track.track.clone(), base));
            tracks.push(&gd_track);
        }
        tracks
    }

    #[func]
    pub fn search_tracks(&mut self, query: GString) -> Array<Gd<GodotTrack>> {
        self.searched_track_idxs.clear();
        if query.is_empty() {
            return self.get_all_tracks();
        }
        let mut out = Array::<Gd<GodotTrack>>::new();
        let library = self.library.as_ref();
        if library.is_none() {
            return out;
        }
        let library = library.unwrap();
        let tracks = library.tracks.iter().map(|t| &t.track);

        let results = search_tracks(&query.to_string(), tracks);

        for (track, _) in results {
            let idx: i32;
            if let Some(library) = &self.library {
                idx = library
                    .tracks
                    .iter()
                    .position(|t| t.track.isrc == track.isrc)
                    .map(|i| i as i32)
                    .unwrap_or(0);
                self.searched_track_idxs.push(idx as i32);
                if let Some(track) = &self.godot_tracks.get(idx as usize) {
                    out.push(track);
                }
            } else {
                let gd_track = Gd::from_init_fn(|base| GodotTrack::from_track(track.clone(), base));
                out.push(&gd_track);
            }
        }
        log::debug!("Found {} tracks", self.searched_track_idxs);
        out
    }

    #[func]
    pub fn get_all_genres(&self) -> Array<GString> {
        let mut out = Array::<GString>::new();
        let library = self.library.as_ref();
        if library.is_none() {
            return out;
        }
        let library = library.unwrap();
        library.tracks.iter().for_each(|track| {
            for genre in &track.track.genre {
                if out.find(genre, 0.into()).is_none() {
                    if !genre.is_empty() { out.push(genre); }
                }
            };
        });

        out.sort_unstable();
        out
    }
    #[func]
    pub fn get_all_custom_tags(&self) -> Array<GString> {
        let mut out = Array::<GString>::new();
        let library = self.library.as_ref();
        if library.is_none() {
            return out;
        }
        let library = library.unwrap();
        library.tracks.iter().for_each(|track| {
            track.track.custom_tags.iter().for_each(|tag| {
                if out.find(&tag.value, 0.into()).is_none() {
                    out.push(&tag.value);
                };
            });
        });

        out
    }

    #[func]
    pub fn find_track_write_genres(&mut self, isrc: String, genres_godot: Array<GString>) -> String {
        let library = self.library.as_mut();
        if library.is_none() {
            return "No library loaded / found.".into();
        }
        let genres = genres_godot.iter_shared().map(|g| g.to_string()).collect::<Vec<_>>();
        let library = library.unwrap();
        if let Some(track) = library.find_track_by_isrc(&isrc) {
            track.track.genre = genres;
            return match track.write() {
                Ok(_) => "".into(),
                Err(e) => e.to_string(),
            };
        }
        "Track not found.".into()
    }
    #[func]
    pub fn find_track_write_custom_tags(&mut self, isrc: String, tags: Array<GString>) -> String {
        let library = self.library.as_mut();
        if library.is_none() {
            return "No library loaded / found.".into();
        }
        let library = library.unwrap();
        if let Some(track) = library.find_track_by_isrc(&isrc) {
            let mut custom_tags: Vec<CustomTag> = Vec::new();
            let string_tags: Vec<String> = tags.iter_shared().map(|s| s.to_string()).collect();
            for string in string_tags {
                custom_tags.push(CustomTag { value: string });
            }
            track.track.custom_tags = custom_tags;
            return match track.write() {
                Ok(_) => "".into(),
                Err(e) => e.to_string(),
            };
        }
        "Track not found.".into()
    }
    #[func]
    pub fn request_track_cover_art(&mut self, isrc: String) -> String {
        let result = {
            let Some(track) = self.library
                .as_mut()
                .and_then(|lib| lib.find_track_by_isrc(&isrc)) 
            else {
                return "Track not found".into();
            };

            if let Some(cover) = &track.track.cover_art {
                Some(Ok(cover.clone()))
            } else {
                Some(Err(track.path.clone()))
            }
        };
        match result {
            Some(Ok(cover)) => {
                self.base_mut().emit_signal("loaded_cover_art", &[Variant::from(isrc.clone()), Variant::from(cover.to_gd_image())]);
            }
            Some(Err(path)) => {
                if let Err(e) = self.cover_request_tx.send(CoverRequest { isrc, path: path }) {
                    return e.to_string().into();
                }
            }
            None => {
                return "Track not found".into()
            }
        };
        "".into()
    }
    #[func]
    pub fn find_track_idx_by_isrc(&self, isrc: String) -> i32 {
        let idx = self
            .library
            .as_ref()
            .and_then(|lib| lib.tracks.iter().position(|track| track.track.isrc == isrc));
        match idx {
            Some(i) => i as i32,
            None => -1,
        }
    }

    #[func]
    pub fn get_track_count(&self) -> i32 {
        self.library
            .as_ref()
            .map(|lib| lib.tracks.len() as i32)
            .unwrap_or(0)
    }
    #[func]
    pub fn get_track_at(&self, idx: i32) -> Option<Gd<GodotTrack>> {
        self.godot_tracks.get(idx as usize)
    }

    #[func]
    pub fn try_load_cache(&mut self) -> bool {
        if let Ok(library) =
            crate::library::cache::load_library(&Path::new(&self.cache_directory.to_string()))
        {
            self.library = Some(library);
            self.godot_tracks = self.get_all_tracks();
            log::debug!("Library loaded from cache successfully!");
            self.base_mut().emit_signal("library_scanned", &[]);
            true
        } else {
            false
        }
    }
    #[func]
    pub fn has_library(&self) -> bool {
        self.library.is_some()
    }
    #[func]
    pub fn scan_directory(&mut self, directory: String) -> String {
        let tx = self.event_tx.clone();
        std::thread::spawn(move || {
            let library = crate::library::scanner::walk_dir(Path::new(&directory), &tx);
            let _ = tx.send(MusicTaggerEvent::Finished(library));
        });

        return "".into();
    }

    #[func]
    pub fn search_list(list_godot: Array<GString>, query: String) -> Array<GString> {
        Array::from_iter(
            list_godot
                .iter_shared()
                .filter(|item| jaro_winkler(&item.to_string(), &query) > 0.6)
        )
    }
}
