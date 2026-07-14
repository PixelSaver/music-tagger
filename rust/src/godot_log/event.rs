use std::path::PathBuf;

use crate::core::models::{Library, TrackPicture};
use crate::error::Result;

pub enum MusicTaggerEvent {
    Scanning(PathBuf),
    TrackFound(String),
    Error(String),
    Finished(Result<Library>),
    LoadedCoverArt(String, Option<TrackPicture>),
}