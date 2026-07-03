use std::{io, path::PathBuf};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, MusicTaggerError>;

#[derive(Debug, Error)]
pub enum MusicTaggerError {
    // filesystem
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("Path is a directory: {0}")]
    PathIsDirectory(PathBuf),

    #[error("Strip prefix error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("Invalid cache")]
    InvalidCache,

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

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

    #[error("Missing tagged file")]
    MissingTaggedFile,

    #[error("Invalid custom tags: {0}")]
    InvalidCustomTags(String),

    // library
    #[error("Track not found")]
    TrackNotFound,

    // listen
    #[error("No active session")]
    NoActiveSession,

    //Playlist
    #[error("Playlist was invalid: {0}")]
    InvalidPlaylist(String),

    //internal
    #[error("{0}")]
    Message(String),
}
