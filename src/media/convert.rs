use std::convert::TryFrom;
use std::str::FromStr;
use lofty::file::{TaggedFile, TaggedFileExt};
use lofty::tag::ItemKey;
use crate::error::*;
use crate::core::models::{Language, Track};

impl TryFrom<TaggedFile> for Track {
    type Error = MusicTaggerError;
    
    fn try_from(file: TaggedFile) -> Result<Self> {
        let tag = file.primary_tag()
            .or_else(|| file.first_tag())
            .ok_or(MusicTaggerError::MissingTag)?;
        Ok(Track {
            track_title: tag
                .get_string(ItemKey::TrackTitle)
                .unwrap_or_default()
                .to_owned(),
            composer: tag
                .get_string(ItemKey::Composer)
                .unwrap_or_default()
                .to_owned(),
            isrc: tag
                .get_string(ItemKey::Isrc)
                .unwrap_or_default()
                .to_owned(),
            track_artist: tag
                .get_string(ItemKey::TrackArtist)
                .unwrap_or_default()
                .to_owned(),
            track_artists: tag
                .get_string(ItemKey::TrackArtists)
                .unwrap_or_default()
                .to_owned(),
            copyright_message: tag
                .get_string(ItemKey::CopyrightMessage)
                .unwrap_or_default()
                .to_owned(),
            description: tag
                .get_string(ItemKey::Description)
                .unwrap_or_default()
                .to_owned(),
            publisher: tag
                .get_string(ItemKey::Publisher)
                .unwrap_or_default()
                .to_owned(),
            script: tag
                .get_string(ItemKey::Script)
                .map(|s| Language::from_str(s))
                .transpose()?
                .to_owned(),
            // TODO get the album 
            album: None,
            genre: tag
                .get_string(ItemKey::Genre)
                .unwrap_or_default()
                .to_owned(),
            duration: tag
                .get_string(ItemKey::Length)
                .unwrap_or_default()
                .to_owned()
                .parse::<u64>()
                .unwrap_or_default(),
        })
    }
}