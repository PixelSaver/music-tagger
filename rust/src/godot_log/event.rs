// use std::path::PathBuf;

use crate::core::models::{Library, TrackPicture};
use crate::error::Result;

pub enum MusicTaggerEvent {
    TrackFound(String),
    Error(String),
    Finished(Result<Library>),
    LoadedCoverArt(String, Option<TrackPicture>),
    ProgressStarted(i32),
    ProgressTick(i32)
}