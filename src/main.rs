use music_tagger::core::models::Playlist;
use music_tagger::playlists::m3u8;
use music_tagger::{library::{cache, scanner}};
use music_tagger::app::app::App;
use music_tagger::app::settings::*;
// use crate::core::models::{Playlist, TrackLocation};




fn main() {
    let mut settings = Settings::new();
    let app = App::new(settings);
    app.run().unwrap();
}
