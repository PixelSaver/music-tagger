mod core;
mod error;
use std::path::PathBuf;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use core::models::Library;
use music_tagger::library::scanner;




fn main() {
    //env_logger::init();
    // let song_file_path: PathBuf = PathBuf::from("./test/music.opus");
    
    let lib = scanner::walk_dir(PathBuf::from("./test/")).unwrap();
    println!("{:?}", lib)
}
