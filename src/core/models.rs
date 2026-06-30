use std::str::FromStr;

use lofty::tag::ItemKey::MusicBrainzRecordingId;

use crate::error::MusicTaggerError;
use crate::error::*;

#[derive(Clone, Debug)]
pub struct Track {
    track_title: String,
    composer: String,
    isrc: Option<Language>,
    track_artist: String,
    track_artists: String,
    copyright_message: String,
    description: String,
    publisher: String,
    /// Language
    script: String,
    album: Option<Album>,
    genre: String,
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