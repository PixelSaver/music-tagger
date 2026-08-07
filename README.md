# Music Tagger

## About 

I like music. This thing makes my music liking easier to manage, especially with huge libraries. Tagging and playlists! 

With a frontend made in Godot and a backend written in rust, this became an exercise in what was possible between the two languages that I knew. 

If you have an issue or bug, please report it and I'll see what I can do. 

## Features

This application can:
 - Support a bunch of filetypes
    - mp3
    - ogg
    - m4a
    - wav
    - flac
- Cache library and only reload manually
- Search the entire library through a lot of filters
  - Genre
  - Fixes
  - Custom Tags
  - Duplicates
- Read / write tags and other metadata
    - Genre
    - Custom Tags
    - Fixes

> ![NOTE] Note
> All editable fields (like the genre, custom tags, and fixes) are editable if you double click the fields. On the first click, it will only display the found tags within the library. 

> ![WARNING] Duplicates
> Since this library finds the tracks by isrc, that means that out of two (or more) duplicate songs, the program will only change one of them, and any changes to that one track will be reflected in every instance of that song in the list. There is CURRENTLY no way to delete duplicates or locate the file location of duplicates.

## Showcase

I'll show a video since screenshots are a little annoying to do, and a lot of the interactions are better shown through video.


https://github.com/user-attachments/assets/60b41330-677d-4d0f-8840-fab00638aac9

 ## Tutorial / Usage
 
 ### DISCLAIMERS

 There are a lot of caveats and shortcuts I have taken because this is not done to a professional degree, and I have not done trial and error / consulted professionals.
 - All custom tags overwrite the `DESCRIPTION` with the following format: `MUSICTAGGER_CUSTOM_TAG:Tag1,Tag2,Tag3`
 - All fixes are custom tags with the prefix: `NEEDSFIX_{fix}`
 - All actions are *irreversible*, `Ctrl+Z` does absolutely NOTHING. You have been warned.

### Get Started

Open the app! On first download, if you click `Explore Library`, it will automatically load the cache. You can also go through the `Setup & Settings` button, then click `Cache / Scan`, and it will load the demo list; then click `Explore Library` from the settings screen.

Once in the library, you can do a lot of things! 

### Tag, Genre, and Fixes Editing

I would first try out the tag editing; select a song by scrolling, and click the tags or genres. Lists should pop up of all tags/genres in the current library, but if you click again on the selection button, it will allow you to write a custom tag/genre. Once you click `enter`, it will save the tag *IMMEDIATELY*. 

The fixes are for if something is wrong with the song. For example, if the lyrics are off, the audio is off, etc. They are essentially a more specific use case of the custom tags, and can be recreated in the custom tags if you want. Still though, I use this to mark which songs are borked, and redownload them from other sources by filtering by tags.

> ![NOTE] Note on filters
> No filter can be reversed (no `does not have artist`). Multiple filters of one category (tags, genres, fixes) are considered `or` (ex: `has genre POP or ALT ROCK`).

### Searching

You can filter by the fixes, tags, genres, etc. Searching using the main search bar fuzzy searches through fields like the title and track artists to filter for songs. There is also a duplicates filter, which cannot delete songs (yet) but is nice to know without searching with your file explorer. 


 ## Roadmap
 - [X] Add tags to songs
    - [ ] Mass update songs in a playlist (add tag to all)
    - [x] Edit and save metadata
        - [x] Custom tags
        - [x] Genre
        - [x] Fixes (for things like incorrect lyrics, metadata, audio, etc)
 - [X] Showing duplicates
 - [X] Get the album cover using Lofty (tag.pictures())
    - [ ] Lazy load album cover so the cache isn't huge for larger libraries
 - [ ] Playlist functionality (not implemented)
    - [ ] Reordering
    - [ ] Showing duplicates
    - [ ] Duration reading from m3u8 for some reason
    - [ ] Upgrade to hls_m3u8 instead of m3u8_rs
- [ ] Find a way to measure length of a file...
- [ ] Automatic cache reloading by watching files
- [ ] Make albums, find them, group files together...
- [ ] Have a `listener mode` 
    - [ ] Reads the current song using system media broadcast (not sure if possible)
    - [ ] Finds the song in the files
    - [ ] Reads / writes metadata (rating, genre)
    - [ ] Flag for redownload (errors like wrong song, lyrics, etc)


### Definitely not going to happen, but I want it to 
 - Album and metadata matching using MusicBrainz
 - Lyrics fetching 
 - Turn into a gdextension to write frontend in godot


## Planning

My personal planning for structure so I don't get lost. Not really important to understanding the project :D

### `app/`

Frontend, no definitions or work, just calling library through cli

### `core/`

Brains of the system, defines structs and reads them out
```rust
Track
Album
Artist
Tags
Library
```

### `media/`

Uses lofty to actually read data 

### `library/`

Walks filesystem, caches result, maybe even watches for filechanges. 
Also stores filepaths (playlists, cache, music libraries)

### `listener/`

Gets the system Now Playing for `listening-mode`

### `playlist/`

Mess with playlists (m3u8_rs)

### `util/`

Only if I have time, for little reusable stuff. Filename renaming, fuzzy helpers, logging. Probably not going to use.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
