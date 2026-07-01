use clap::Parser;
use crate::app::cli::*;
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
    pub fn run(&self) -> Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Commands::Scan { music_directory, cache_directory } => {
                self.scan(&music_directory, &cache_directory)?;
            },
            Commands::Playlist { playlist_name, playlist_directory } => {
                self.playlist(&playlist_name, &playlist_directory)?;
            }
            Commands::Tag { isrc, mode } => {
                self.tag(&isrc, mode)?;
            },
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
    fn tag(&self, isrc: &str, mode: TagMode) -> Result<()> {
        let library = self.load_or_scan_library(&self.settings.cache_directory)?;
        if let Some(library) = &self.library {
            
        } else {
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
        }
        match mode {
            TagMode::Append => {
                
            },
            TagMode::Replace => {
                
            },
            TagMode::Remove => {
                
            }
        }
        Ok(())
    }
    fn load_or_scan_library(&self, cache_directory: &Path) -> Result<Library> {
        let cached_library = cache::load_library(cache_directory)?;
        if let Some(library) = cached_library {
            Ok(library)
        } else {
            Ok(self.scan(&self.settings.music_directories[0], cache_directory)?)
        }
    }
}