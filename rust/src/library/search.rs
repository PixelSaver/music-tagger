use strsim::jaro_winkler;
use crate::core::models::*;

pub fn search_tracks<'a, I> 
(query: &str, tracks: I) -> Vec<(&'a Track, f64)>
where I: IntoIterator<Item = &'a Track>
{
    let query = query.to_lowercase();
    let mut results: Vec<_> = tracks.into_iter()
        .map(|track| {
            let title = track.track_title.as_str().to_lowercase();
            let genres = track.genre.as_str().to_lowercase();
            let custom_tags = track.custom_tags.iter().map(|tag| tag.value.to_lowercase()).collect::<Vec<_>>();
            let artist = track.track_artist.as_str().to_lowercase();
            let total = title + " " + &genres + " " + " " + &artist + " " + &custom_tags.join(" ");
            
            let score = if total.contains(&query) {
                1.0
            } else {
                jaro_winkler(&total, &query)
            };
    
            (track, score)
        })
        .filter(|(_, score)| *score > 0.5)
        .collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
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