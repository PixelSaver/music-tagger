use std::path::Path;

pub enum MusicTaggerEvent<'a> {
    Scanning(&'a Path),
    TrackFound(&'a str),
    Error(&'a str),
}

pub trait EventReporter {
    fn emit(&mut self, event: MusicTaggerEvent);
}