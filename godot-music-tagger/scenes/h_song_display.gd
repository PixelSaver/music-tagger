extends Panel
class_name HTrackDisplay

const TAG = preload("res://scenes/tag.tscn")

@export var label: RichTextLabel 
@export var tags_cont: Control

func _clear_tags() -> void:
	for child in tags_cont.get_children():
		child.queue_free()

func display_track(track:GodotTrack):
	label.text = track.track_title
	
	if tags_cont.get_children().size() > 0:
		_clear_tags()
		#TODO Maybe reuse tags so I don't clear them every time in tag display
	for genre in track.genres:
		if genre.is_empty(): continue
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(genre, Color("1a1a1a99"))
		tags_cont.add_child(t)
	for tag in track.custom_tags:
		if tag.is_empty(): continue
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(tag, Color("1a1a1a99"))
		tags_cont.add_child(t)
	for fix in track.get_fixes():
		if fix.is_empty(): continue
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(fix, Color.CRIMSON)
		tags_cont.add_child(t)
		
