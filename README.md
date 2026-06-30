# Music Tagger

## About 

I like music. This thing makes my music liking easier to manage, especially with huge libraries. Tagging and playlists! 

## Features
This cli (might consider tui or gui later) app ~~can~~ will:
 - Support a bunch of filetypes
    - mp3
    - ogg
    - m4a
    - wav
    - flac
- Easily add and remove from playlists
- Cache searches and only reload periodically / manually
- Read / write tags and other metadata
    - Genre
    - Language
    - Etc
- Have a `listener mode` 
    - Reads the current song using system media broadcast (not sure if possible)
    - Finds the song in the files
    - Reads / writes metadata (rating, genre)
    - Flag for redownload (errors like wrong song, lyrics, etc)
### Definitely not going to happen, but I want it to 
 - Album and metadata matching using MusicBrainz
 - Lyrics fetching 
 - Turn into a gdextension to write frontend in godot

 ## Roadmap
 - [ ] Add tags to playlists / songs
    - [ ] Mass update songs in a playlist (add tag to all)
 - [ ] Playlist functionality
    - [ ] Reordering
    - [ ] Removing duplicates
    - [ ] Duration reading from m3u8 for some reason

## Planning

My personal planning for structure so I don't get lost

## `app/`

Frontend, no definitions or work, just calling library through cli

## `core/`

Brains of the system, defines structs and reads them out
```rust
Track
Album
Artist
Tags
Library
```

## `media/`

Uses lofty to actually read data 

## `library/`

Walks filesystem, caches result, maybe even watches for filechanges. 
Also stores filepaths (playlists, cache, music libraries)

## `listener/`

Gets the system Now Playing for `listening-mode`

## `playlist/`

Mess with playlists (m3u8_rs)

## `util/`

Only if I have time, for little reusable stuff. Filename renaming, fuzzy helpers, logging. Probably not going to use.