use std::{io, path::PathBuf};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MusicTaggerError>;

#[derive(Debug, Error)]
pub enum MusicTaggerError {
    // filesystem
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    // metadata
    #[error(transparent)]
    Metadata(#[from] lofty::error::LoftyError),

    #[error("Unsupported file type")]
    UnsupportedFileType,

    #[error("Invalid language: {0}")]
    InvalidLanguage(String),

    #[error("Missing tag")]
    MissingTag,

    // library
    #[error("Track not found")]
    TrackNotFound,

    #[error("Invalid cache")]
    InvalidCache,

    // listen
    #[error("No active session")]
    NoActiveSession,

    //Playlist
    #[error("Invalid playlist")]
    InvalidPlaylist,

    //internal
    #[error("{0}")]
    Message(String),
}
