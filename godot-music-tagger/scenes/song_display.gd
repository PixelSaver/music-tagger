extends Panel
class_name SongDisplayPanel

@export var title: RichTextLabel
@export var cover: TextureRect
@export var desc: RichTextLabel

func display_track(track:GodotTrack) -> void:
	title.text = track.track_title
	desc.text = ""
	if track.track_artist.length() > 0:
		desc.text += "Artist: %s\n" % track.track_artist
	if track.composer.length() > 0:
		desc.text += "Composer: %s\n" % track.composer
	if track.genre.length() > 0:
		desc.text += "Genre: %s\n" % track.genre
	if track.custom_tags.size() > 0:
		desc.text += "Custom Tags: "
		for tag in track.custom_tags:
			if tag.length() <= 0: continue
			desc.text += "%s," % tag
		desc.text += "\n"
	
