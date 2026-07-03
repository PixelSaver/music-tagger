use crate::error::*;
use std::fs::File;
use lofty;
use crate::core::models::Track;


pub fn read_track_from_file(file: &mut File) -> Result<Track> {
    let audio_file = lofty::read_from(file)?;
    Ok(audio_file.try_into()?)
}