mod error;
use std::path::PathBuf;
use lofty::file::TaggedFileExt;
use lofty::probe::Probe;


fn main() {
    let song_file_path: PathBuf = PathBuf::from("./test/music.opus");

    let file = lofty::read_from_path(song_file_path.clone());
    let guessed_file = Probe::open(song_file_path).unwrap().guess_file_type().unwrap().read().unwrap();
    for tag in guessed_file.tags() {
        println!("Tag: {:?}", tag.tag_type());
        for item in tag.items() {
            println!("{:?}: {:?}", item.key(), item.value());
        }
    }
}
