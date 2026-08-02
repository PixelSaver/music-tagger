// use strsim::jaro_winkler;
use nucleo_matcher::{Matcher, Config};
use nucleo_matcher::Utf32Str;
use crate::core::models::*;


pub fn search_tracks<'a, I>(
    query: &str, 
    selected_tags: &[String],
    selected_genres: &[String],
    selected_fixes: &[String],
    tracks: I
) -> Vec<(&'a Track, f64)>
where I: IntoIterator<Item = &'a Track>
{
    let mut matcher = Matcher::new(Config::DEFAULT);
    let mut buf = Vec::new();
    let mut buf_1 = Vec::new();
    let mut buf_2 = Vec::new();
    let query = Utf32Str::new(query, &mut buf);
    let mut results: Vec<_> = tracks.into_iter()
        .filter(|track| {
            let genre_ok = selected_genres.is_empty()
                || selected_genres.iter().any(|selected_genre| track.genre.iter().any(|genre| genre.eq_ignore_ascii_case(selected_genre)));
            let tags_ok = selected_tags.is_empty()
                || selected_tags.iter().any(|selected_tag| track.custom_tags.iter().any(|tag| tag.value.eq_ignore_ascii_case(selected_tag)));
            let fixes_ok = selected_fixes.is_empty()
                || selected_fixes.iter().any(|selected_fix| track.custom_tags.iter().any(|tag| tag.value.eq_ignore_ascii_case(selected_fix)));
            genre_ok && tags_ok && fixes_ok
        })
        .map(|track| {
            
            let score = if query.is_empty() {
                1.0
            } else {
                let score = [
                    matcher.fuzzy_match(Utf32Str::new(&track.track_title, &mut buf_1), query),
                    matcher.fuzzy_match(Utf32Str::new(&track.track_artist, &mut buf_2), query),
                ]
                .into_iter()
                .flatten()
                .max()
                .unwrap_or(0.0 as u16) as f64;
                score
                // let title = track.track_title.as_str().to_lowercase();
                // let genres = track.genre.join(" ").to_lowercase();
                // let custom_tags = track.custom_tags.iter().map(|tag| tag.value.to_lowercase()).collect::<Vec<_>>();
                // let artist = track.track_artist.as_str().to_lowercase();
                // let total = format!(
                //     "{} {} {} {}",
                //     title,
                //     genres,
                //     custom_tags.join(" "),
                //     artist
                // );
                
                // if total.contains(&query) {
                //     1.0
                // } else {
                //     jaro_winkler(&total, &query)
                // }
            };
    
            (track, score)
        })
        .filter(|(_, score)| *score > 0.5)
        .collect();
    results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
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