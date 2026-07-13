use std::path::Path;
use std::str::FromStr;
use lofty::file::{AudioFile, TaggedFile, TaggedFileExt};
use lofty::picture::PictureType;
use lofty::tag::{ItemKey};
use lofty::config::WriteOptions;
use crate::error::*;
use crate::core::models::{CustomTag, Language, Track, TrackLocation, TrackPicture};
// use crate::media::tag;
use std::fs::OpenOptions;

impl Track {
    pub fn to_tagged_file(&self, path: &Path) -> Result<TaggedFile> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mut tagged_file = lofty::read_from(&mut file)?;
        let tag = if let Some(tag) = tagged_file.primary_tag_mut() {
            tag
        } else if let Some(tag) = tagged_file.first_tag_mut() {
            tag
        } else {
            return Err(MusicTaggerError::MissingTag)
        };
        
        tag.insert_text(ItemKey::TrackTitle, self.track_title.clone());
        tag.insert_text(ItemKey::Composer, self.composer.clone());
        tag.insert_text(ItemKey::Isrc, self.isrc.clone());
        tag.insert_text(ItemKey::TrackArtist, self.track_artist.clone());
        tag.insert_text(ItemKey::TrackArtists, self.track_artists.clone());
        tag.insert_text(ItemKey::CopyrightMessage, self.copyright_message.clone());
        tag.insert_text(ItemKey::Publisher, self.publisher.clone());
        tag.insert_text(ItemKey::Genre, self.genre.clone());
        
        tag.insert_text(
            ItemKey::Description,
            CustomTag::vec_to_str(&self.custom_tags),
        );
        
        if let Some(lang) = self.script.clone() {
            tag.insert_text(ItemKey::Script, lang.to_string());
        }
        Ok(tagged_file)
    }

    pub fn from_tagged_file(file: &TaggedFile) -> Result<Self> {
        let tag = file.primary_tag()
            .or_else(|| file.first_tag())
            .ok_or(MusicTaggerError::MissingTag)?;
        let cover = tag
            .pictures()
            .iter()
            .find(|p| p.pic_type() == PictureType::CoverFront)
            .or_else(|| tag.pictures().first())
            .map(|p| TrackPicture {
                data: p.data().to_vec(),
                mime_type: p.mime_type().map(|s| s.to_string()),
            });
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
            cover_art: cover,
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
            // lofty_tagged_file: Some(file),
        })
    }
}


impl TrackLocation {
    pub fn get_cover_art(&mut self) -> Result<TrackPicture> {
        if let Some(cover_art) = self.track.cover_art.clone() {
            return Ok(cover_art);
        } else {
            self.get_file()?;
            let file = self.lofty_tagged_file.as_ref().ok_or(MusicTaggerError::MissingTaggedFile)?;
            let tag = file.primary_tag()
                .or_else(|| file.first_tag())
                .ok_or(MusicTaggerError::MissingTag)?;
            let cover = tag
                .pictures()
                .iter()
                .find(|p| p.pic_type() == PictureType::CoverFront)
                .or_else(|| tag.pictures().first())
                .map(|p| TrackPicture {
                    data: p.data().to_vec(),
                    mime_type: p.mime_type().map(|s| s.to_string()),
                });
            if cover.is_none() {
                return Err(MusicTaggerError::MissingTag);
            }
            let unwrapped_cover = cover.unwrap();
            self.track.cover_art = Some(unwrapped_cover.clone());
            Ok(unwrapped_cover)
        }
    }

    pub fn write(&mut self) -> Result<()> {
        self.get_file()?;
        let path = &self.path;
        let tagged_file: TaggedFile = (&mut self.track).to_tagged_file(path)?;
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
        self.lofty_tagged_file = Some(lofty::read_from_path(&self.path)?);
        Ok(())
    }
}