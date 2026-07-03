use env_logger::Env;
use music_tagger::app::app::App;
use music_tagger::app::settings::*;
// use crate::core::models::{Playlist, TrackLocation};




fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    let mut settings = Settings::new();
    settings.music_directories = vec!["./test".into()];
    settings.cache_directory = "./test/.cache/library.json".into();
    let mut app = App::new(settings);
    app.run().unwrap();
}
