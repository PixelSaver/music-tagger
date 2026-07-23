use crate::core::models::*;
use std::collections::HashMap;

pub fn check_duplicates(library: &Library) -> HashMap<String, Vec<TrackLocation>> {
    let mut first_dupe: HashMap<String, Vec<TrackLocation>> = HashMap::new();
    
    for track in &library.tracks {
        let key = track.track.isrc.clone();
        first_dupe.entry(key).or_default().push(track.clone());
    }
    let mut duplicates: HashMap<String, Vec<TrackLocation>> = HashMap::new();
    for (_, dupe) in first_dupe.iter_mut() {
        if dupe.len() > 1 {
            duplicates.insert(dupe[0].track.isrc.clone(), dupe.clone());
        }
    }

    duplicates
}