use std::str::FromStr;
use std::path::PathBuf;
use crate::error::MusicTaggerError;
use crate::error::*;

#[derive(Clone, Debug)]
pub struct Track {
    pub track_title: String,
    pub composer: String,
    pub isrc: Option<Language>,
    pub track_artist: String,
    pub track_artists: String,
    pub copyright_message: String,
    pub description: String,
    pub publisher: String,
    /// Language
    pub script: String,
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
            script: String::new(),
            album: Some(Album::new()),
            genre: String::new(),
            isrc: Option::None,
        }
    }
}
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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


#[derive(Debug)]
pub struct TrackLocation {
    pub track: Track,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Library {
    pub tracks: Vec<TrackLocation>,
}