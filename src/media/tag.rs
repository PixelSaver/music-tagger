use crate::core::models::*;
use crate::error::*;

pub fn tag_track(track: &mut Track, _mode: TagMode, tag: CustomTag) -> Result<()> {
    // match mode {
    //     TagMode::Append => {
            
    //     },
    //     TagMode::Replace => {
            
    //     },
    //     TagMode::Remove => {
            
    //     },
    // }
    track.custom_tags.push(tag);
    Ok(())
}