extends Panel
class_name HTrackDisplay

const TAG = preload("res://scenes/tag.tscn")

@export var label: RichTextLabel 
@export var tags_cont: Control
var _track: GodotTrack
var _box: StyleBoxFlat

func _ready() -> void:
	var panel = self.get_theme_stylebox("panel").duplicate()
	if not panel:
		panel = StyleBoxFlat.new()
	_box = panel
	self.add_theme_stylebox_override("panel", _box)
	

func _clear_tags() -> void:
	for child in tags_cont.get_children():
		child.queue_free()

func set_display_palette(palette:Array[Color]) -> void:
	if palette.size() < 2:
		palette = [Color("1a1a1a"), Color.WHITE]
	_box.bg_color = palette[0]
	_box.bg_color.a = 0.6
	label.add_theme_color_override("default_color", palette[1])

func display_track(track:GodotTrack):
	if track == _track:
		return
	_track = track
	label.text = track.track_title
	set_display_palette(track.palette)
	
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
		
