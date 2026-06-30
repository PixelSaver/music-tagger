use std::str::FromStr;
use std::path::PathBuf;
use crate::error::MusicTaggerError;
use crate::error::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
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
            _ => Err(MusicTaggerError::InvalidLanguage(s.to_owned())),
        }
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