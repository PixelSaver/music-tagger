use crate::error::MusicTaggerError;
use clap::Parser;
use crate::app::cli::*;
use crate::media::tag;
use std::path::Path;
use std::{fs, io::{self, Write}};
use crate::error::Result;
use crate::app::settings::Settings;
use crate::library::{scanner, cache};
use crate::core::models::*;

pub struct App {
    settings: Settings,
    library: Option<Library>,
}
impl App {
    pub fn new(settings: Settings) -> Self {
        Self { 
            settings,
            library: None,
        }
    }
    pub fn run(&mut self) -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan { music_directory, cache_directory } => {
                log::debug!("Scanning music directory: {:?}", music_directory);
                log::debug!("Cache directory: {:?}", cache_directory);
                self.scan(&music_directory, &cache_directory)?;
            },
            Commands::Playlist { playlist_name, playlist_directory } => {
                self.playlist(&playlist_name, &playlist_directory)?;
            }
            Commands::Tag { isrc, mode, tag } => {
                self.tag(&isrc, mode, CustomTag { value: tag })?;
            },
            Commands::Inspect { isrc } => {
                self.inspect(&isrc)?;
            },
        }
        
        Ok(())
    }
    fn inspect(&self, isrc: &str) -> Result<()> {
        let library = self.load_or_scan_library(&self.settings.cache_directory)?;
        log::info!("Library: {:?}", library);
        let track = library.tracks.iter().find(|t| t.track.isrc == isrc);
        if let Some(track) = track {
            log::info!("{:?}", track.track);
        }
        Ok(())
    }
    fn scan(&self, music_directory: &Path, cache_directory: &Path) -> Result<Library> {
        let library = scanner::walk_dir(music_directory)?;
        cache::save_library(&cache_directory, &library)?;
        Ok(library)
    }
    fn playlist(&self, playlist_name: &str, playlist_directory: &Path) -> Result<()> {
        Ok(())
    }
    fn tag(&mut self, isrc: &str, _mode: TagMode, tag: CustomTag) -> Result<()> {
        if let Ok(library) = self.load_or_scan_library(&self.settings.cache_directory) {
            self.library = Some(library);
        }else {
            println!("\nNo cache found. Scan default directory? [Y/n] ");
            io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            
            if !response.trim().eq_ignore_ascii_case("n") {
                println!("Cancelled.");
                return Ok(());
            }
            
            let library = scanner::walk_dir(&self.settings.music_directories[0])?;
            cache::save_library(&self.settings.cache_directory, &library)?;
            self.library = Some(library);
        }
        if let Some(library) = &mut self.library {
            let track_loc: &mut TrackLocation = library.find_track_by_isrc(isrc).ok_or(MusicTaggerError::TrackNotFound)?;
            tag::tag_track(&mut track_loc.track, _mode, tag)?;
            track_loc.write()?;
        }
        
        
        Ok(())
    }
    fn load_or_scan_library(&self, cache_directory: &Path) -> Result<Library> {
        log::debug!("Loading or scanning library path: {:?}", cache_directory);
        let cached_library = cache::load_library(cache_directory)?;
        if let Some(library) = cached_library {
            Ok(library)
        } else {
            Ok(self.scan(&self.settings.music_directories[0], cache_directory)?)
        }
    }
}