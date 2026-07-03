use clap::ValueEnum;
use lofty::file::TaggedFile;
use std::str::FromStr;
use std::path::PathBuf;
use crate::error::MusicTaggerError;
use crate::error::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub track_title: String,
    pub composer: String,
    pub isrc: String,
    pub track_artist: String,
    pub track_artists: String,
    pub copyright_message: String,
    pub description: String,
    pub publisher: String,
    /// Language
    pub script: Option<Language>,
    pub album: Option<Album>,
    pub genre: String,
    /// Track length in milliseconds
    pub duration: u64,
    pub custom_tags: Vec<CustomTag>,
    #[serde(skip)]
    pub lofty_tagged_file: Option<TaggedFile>,
}
impl std::fmt::Debug for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Track")
            .field("track_title", &self.track_title)
            .field("composer", &self.composer)
            .field("track_artist", &self.track_artist)
            .field("track_artists", &self.track_artists)
            .field("copyright_message", &self.copyright_message)
            .field("description", &self.description)
            .field("publisher", &self.publisher)
            .field("script", &self.script)
            .field("album", &self.album)
            .field("genre", &self.genre)
            .field("isrc", &self.isrc)
            .field("duration", &self.duration)
            .field("custom_tags", &self.custom_tags)
            .finish()
    }
}
impl Track {
    pub fn new() -> Self {
        Self {
            track_title: String::new(),
            composer: String::new(),
            track_artist: String::new(),
            track_artists: String::new(),
            copyright_message: String::new(),
            description: String::new(),
            publisher: String::new(),
            script: Option::None,
            album: Some(Album::new()),
            genre: String::new(),
            isrc: String::new(),
            duration: 0,
            custom_tags: Vec::new(),
            lofty_tagged_file: Option::None,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    album_artist: String,
    album_title: String,
}
impl Album {
    pub fn new() -> Self {
        Self {
            album_artist: String::new(),
            album_title: String::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
    Japanese,
    Korean,
    None,
}
impl FromStr for Language {
    type Err = MusicTaggerError;

    fn from_str(s: &str) -> Result<Self> {
        let str = s.to_lowercase();
        match str.as_str() {
            "eng" => Ok(Self::English),
            "spa" => Ok(Self::Spanish),
            "span" => Ok(Self::Spanish),
            "jpn" => Ok(Self::Japanese),
            "jpan" => Ok(Self::Japanese),
            "kor" => Ok(Self::Korean),
            "kore" => Ok(Self::Korean),
            // _ => Err(MusicTaggerError::InvalidLanguage(s.to_owned())),
            _ => Ok(Self::None),
        }
    }
}
impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::English => "eng".to_string(),
            Self::Spanish => "spa".to_string(),
            Self::Japanese => "jpn".to_string(),
            Self::Korean => "kor".to_string(),
            Self::None => "".to_string(),
        }
    }
}

#[derive(ValueEnum, Clone)]
pub enum TagMode {
    Append,
    Replace,
    Remove,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomTag {
    pub value: String,
}
impl CustomTag {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn vec_to_str(vec: &[CustomTag]) -> String {
        let mut out = "MUSICTAGGER_CUSTOM_TAGS:".to_string();
        for tag in vec {
            out.push_str(&tag.value);
            out.push_str(",");
        }
        out
    }
    pub fn str_to_vec(s: &str) -> Result<Vec<CustomTag>> {
        if !s.starts_with("MUSICTAGGER_CUSTOM_TAGS:") {
            return Err(MusicTaggerError::InvalidCustomTags(s.to_owned()));
        }
        let mut out = Vec::new();
        for tag in s.split(',') {
            out.push(CustomTag::new(tag.to_owned()));
        }
        Ok(out)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackLocation {
    pub track: Track,
    pub path: PathBuf,
}
impl TrackLocation {
    pub fn new(track: Track, path: PathBuf) -> Self {
        Self { track, path }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    pub tracks: Vec<TrackLocation>,
}
impl Library {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }
    
    pub fn add_track(&mut self, track: TrackLocation) {
        if self.tracks.iter().any(|t| t.path == track.path) {
            return;
        }
        self.tracks.push(track);
    }

    pub fn find_track_by_isrc(&mut self, isrc: &str) -> Option<&mut TrackLocation> {
        self.tracks.iter_mut().find(|t| t.track.isrc == isrc)
    }
    pub fn find_track_by_title(&mut self, title: &str) -> Option<&mut TrackLocation> {
        self.tracks.iter_mut().find(|t| t.track.track_title == title)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// TODO Add tags
pub struct Playlist {
    pub playlist_name: String,
    pub track_locations: Vec<TrackLocation>,
    // tags: String,
}
impl Playlist {
    pub fn new() -> Self {
        Self {
            playlist_name: String::new(),
            track_locations: Vec::new(),
        }
    }
    pub fn add_track(&mut self, track: TrackLocation) {
        self.track_locations.push(track);
    }
}