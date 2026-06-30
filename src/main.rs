use music_tagger::core::models::Playlist;
use music_tagger::error;
use music_tagger::playlists::m3u8;
use music_tagger::{library::{cache, scanner}};
// use crate::core::models::{Playlist, TrackLocation};




fn main() {
    //env_logger::init();
    // let song_file_path: PathBuf = PathBuf::from("./test/music.opus");
    let cache_path = "./test/.cache/library.json";
    let lib = scanner::walk_dir("./test/".as_ref()).unwrap();
    cache::save_library(cache_path.as_ref(), &lib).unwrap();
    let lib = cache::load_library(cache_path.as_ref()).unwrap().expect("Library wasn't good");
    println!("{:?}", lib);
    
    let mut playlist = Playlist::new();
    for track in lib.tracks {
        playlist.add_track(track);
    }
    m3u8::save_m3u8("./test/playlists/playlist.m3u8".as_ref(), &playlist).unwrap();
    println!("{:?}", playlist);
}
