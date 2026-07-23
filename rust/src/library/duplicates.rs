use crate::core::models::*;
use std::collections::HashMap;

pub fn check_duplicates(library: &Library) -> HashMap<String, Vec<TrackLocation>> {
    let mut duplicates: HashMap<String, Vec<TrackLocation>> = HashMap::new();
    
    for track in &library.tracks {
        let key = track.track.isrc.clone();
        duplicates.entry(key).or_default().push(track.clone());
    }

    duplicates
}