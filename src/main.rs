mod core;
mod error;
use std::path::Path;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;
use core::models::Library;
use music_tagger::library::{cache, scanner};




fn main() {
    //env_logger::init();
    // let song_file_path: PathBuf = PathBuf::from("./test/music.opus");
    let cache_path = "./test/.cache/";
    let lib = scanner::walk_dir("./test/".as_ref()).unwrap();
    cache::store_cache(cache_path.as_ref(), &lib).unwrap();
    let lib = cache::load_cache(cache_path.as_ref()).unwrap();
    println!("{:?}", lib)
}
