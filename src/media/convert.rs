use std::convert::TryFrom;
use std::str::FromStr;
use lofty::file::{TaggedFile, TaggedFileExt};
use lofty::tag::{ItemKey, TagItem};
use crate::error::*;
use crate::core::models::{CustomTag, Language, Track};

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
            custom_tags: CustomTag::str_to_vec(
                    tag
                    .get_string(ItemKey::Description)
                    .unwrap_or_default(),
                )?,
            lofty_tagged_file: Some(file),
        })
    }
}
impl TryInto<TaggedFile> for Track {
    type Error = MusicTaggerError;

    fn try_into(mut self) -> Result<TaggedFile> {
        let tagged_file = self
            .lofty_tagged_file
            .as_mut()
            .ok_or(MusicTaggerError::MissingTaggedFile)?;

        let tag = if let Some(tag) = tagged_file.primary_tag_mut() {
            tag
        } else if let Some(tag) = tagged_file.first_tag_mut() {
            tag
        } else {
            return Err(MusicTaggerError::MissingTag)
        };

        tag.insert_text(ItemKey::TrackTitle, self.track_title);
        tag.insert_text(ItemKey::Composer, self.composer);
        tag.insert_text(ItemKey::Isrc, self.isrc);
        tag.insert_text(ItemKey::TrackArtist, self.track_artist);
        tag.insert_text(ItemKey::TrackArtists, self.track_artists);
        tag.insert_text(ItemKey::CopyrightMessage, self.copyright_message);
        tag.insert_text(ItemKey::Publisher, self.publisher);
        tag.insert_text(ItemKey::Genre, self.genre);

        tag.insert_text(
            ItemKey::Description,
            CustomTag::vec_to_str(self.custom_tags),
        );

        if let Some(lang) = self.script {
            tag.insert_text(ItemKey::Script, lang.to_string());
        }

        Ok(self.lofty_tagged_file.take().unwrap())
    }
}