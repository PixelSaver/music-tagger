use crate::error::*;
use std::fs::File;
use lofty::{self, file::TaggedFile};
use crate::core::models::Track;


pub fn read_track_from_file(file: &mut File) -> Result<(Track, TaggedFile)> {
    let audio_file = lofty::read_from(file)?;
    let track = Track::from_tagged_file(&audio_file)?;
    Ok((track, audio_file))
}