use clap::{Parser, Subcommand, ValueEnum};
use crate::core::models::TagMode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rust-music-tagger")]
#[command(about = "A cli tool to tag music interactively", long_about = None)]
#[command(version)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
  Scan {
      #[arg(short='m', long, default_value = "./test/")]
      music_directory: PathBuf,

      #[arg(short='c', long, default_value = "./test/.cache/library.json")]
      cache_directory: PathBuf,
  },
  /// Export playlist??
  Playlist {
      #[arg(short='p', long, default_value = "playlist")]
      playlist_name: String,

      #[arg(short='d', long, default_value = ".")]
      playlist_directory: PathBuf,
  },
  Tag {
      #[arg(short='i', long, default_value = "")]
      isrc: String,

      #[arg(short='m', long, default_value = "replace")]
      mode: TagMode,
  },
  Inspect {
      #[arg(short='p', long, default_value = "./test/")]
      path: PathBuf,
  }
}