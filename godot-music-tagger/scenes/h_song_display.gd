extends Panel
class_name HTrackDisplay

const TAG = preload("res://scenes/tag.tscn")

@export var label: RichTextLabel 
@export var tags_cont: Control


func display_track(track:GodotTrack):
	label.text = track.track_title
	for genre in track.genres:
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(genre, Color.DARK_GRAY)
	for tag in track.custom_tags:
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(tag, Color.DARK_GRAY)
	for fix in track.get_fixes():
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(fix, Color.CRIMSON)
		
