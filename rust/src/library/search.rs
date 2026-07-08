use strsim::jaro_winkler;
use crate::core::models::*;

pub fn search_tracks<'a, I> 
(query: &str, tracks: I) -> Vec<(&'a Track, f64)>
where I: IntoIterator<Item = &'a Track>
{
    tracks.into_iter()
        .map(|track| {
            let score = jaro_winkler(&track.track_title, query);
            (track, score)
        })
        .collect()
    // let query = query.to_lowercase();
    // let mut results: Vec<_> = tracks
    //     .iter()
    //     .map(|track| {
    //         let track_title = track.track_title.to_lowercase();
    //         let score = if track_title.contains(&query) {
    //             1.0
    //         } else {
    //             jaro_winkler(&track_title, &query)
    //         };
    //         (track, score)
    //     })
    //     .filter(|(_, score)| *score > 0.5)
    //     .collect();
    // results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    // results
}