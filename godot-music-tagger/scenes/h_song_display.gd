extends Panel
class_name HTrackDisplay

const TAG = preload("res://scenes/tag.tscn")

@export var label: RichTextLabel 
@export var tags_cont: Control
var _track: GodotTrack
var _box: StyleBoxFlat
var _palette: Array[Color] = []

func _ready() -> void:
	get_viewport().size_changed.connect(_update_size)
	_update_size()
	var panel = self.get_theme_stylebox("panel").duplicate()
	if not panel:
		panel = StyleBoxFlat.new()
	_box = panel
	self.add_theme_stylebox_override("panel", _box)

func _update_size():
	self.custom_minimum_size = Vector2(get_viewport_rect().size.x*1.2 - 400, 100)

func _clear_tags() -> void:
	for child in tags_cont.get_children():
		child.queue_free()

func set_display_palette(palette:Array[Color]) -> void:
	if palette.size() < 2:
		palette = [Color("bec0c0ff"), Color.WHITE]
	_palette = palette
	if not _box:
		return
	var panel_col = palette[1]
	panel_col.ok_hsl_l = clampf(panel_col.ok_hsl_l, 0.2, 0.4)
	_box.bg_color = panel_col
	_box.bg_color.a = 0.6
	#$ColorRect.color = palette[1]
	var text_col = palette[0]
	text_col.ok_hsl_s = clampf(text_col.ok_hsl_s, 0.0, 0.8)
	text_col.v = clampf(text_col.v, 0.8, 1.0)
	label.add_theme_color_override("default_color", text_col)

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
		t.set_tag(genre, Color("c4f0c2ff"))
		tags_cont.add_child(t)
	for tag in track.custom_tags:
		if tag.is_empty(): continue
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(tag, Color("ffa7b9ff"))
		tags_cont.add_child(t)
	for fix in track.get_fixes():
		if fix.is_empty(): continue
		var t = TAG.instantiate() as TagDisplay
		t.set_tag(fix, Color.CRIMSON)
		tags_cont.add_child(t)
		
