use std::convert::TryFrom;
use std::str::FromStr;
use lofty::file::{AudioFile, TaggedFile, TaggedFileExt};
use lofty::tag::{ItemKey};
use lofty::config::WriteOptions;
use crate::error::*;
use crate::core::models::{CustomTag, Language, Track, TrackLocation};
use crate::media::tag;
use std::fs::{self, OpenOptions};

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
            custom_tags: match CustomTag::str_to_vec(
                tag.get_string(ItemKey::Description)
                    .unwrap_or_default(),
            ) {
                Ok(description) => description,
                Err(_) => Vec::new(),
            },
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
            CustomTag::vec_to_str(&self.custom_tags),
        );

        if let Some(lang) = self.script {
            tag.insert_text(ItemKey::Script, lang.to_string());
        }

        Ok(self.lofty_tagged_file.take().unwrap())
    }
}
impl<'a> TryFrom<&'a mut Track> for &'a mut TaggedFile {
    type Error = MusicTaggerError;

    fn try_from(track: &'a mut Track) -> Result<Self> {
        let tagged_file = track
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
        
        tag.insert_text(ItemKey::TrackTitle, track.track_title.clone());
        tag.insert_text(ItemKey::Composer, track.composer.clone());
        tag.insert_text(ItemKey::Isrc, track.isrc.clone());
        tag.insert_text(ItemKey::TrackArtist, track.track_artist.clone());
        tag.insert_text(ItemKey::TrackArtists, track.track_artists.clone());
        tag.insert_text(ItemKey::CopyrightMessage, track.copyright_message.clone());
        tag.insert_text(ItemKey::Publisher, track.publisher.clone());
        tag.insert_text(ItemKey::Genre, track.genre.clone());
        
        tag.insert_text(
            ItemKey::Description,
            CustomTag::vec_to_str(&track.custom_tags),
        );
        
        if let Some(lang) = track.script.clone() {
            tag.insert_text(ItemKey::Script, lang.to_string());
        }
        
        Ok(tagged_file)
    }
}

impl TrackLocation {
    pub fn write(&mut self) -> Result<()> {
        self.get_file()?;
        let path = &self.path;
        let tagged_file: &mut TaggedFile = (&mut self.track).try_into()?;
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;
        log::debug!("Writing file: {:?}", path);
        if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
            log::debug!(
                "Description: {:?}",
                tag.get_string(ItemKey::Description)
            );
        }
        tagged_file.save_to(&mut file, WriteOptions::default())?;
        Ok(())
    }
    pub fn get_file(&mut self) -> Result<()> {
        log::debug!("Getting file: {:?}", self.path);
        self.track.lofty_tagged_file = Some(lofty::read_from_path(&self.path)?);
        Ok(())
    }
}