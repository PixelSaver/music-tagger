use env_logger::Env;
use music_tagger::core::models::Playlist;
use music_tagger::playlists::m3u8;
use music_tagger::{library::{cache, scanner}};
use music_tagger::app::app::App;
use music_tagger::app::settings::*;
// use crate::core::models::{Playlist, TrackLocation};




fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let mut settings = Settings::new();
    settings.music_directories = vec!["~/Music/spotiflacopus".into()];
    settings.cache_directory = "./test/.cache/library.json".into();
    let app = App::new(settings);
    app.run().unwrap();
}
