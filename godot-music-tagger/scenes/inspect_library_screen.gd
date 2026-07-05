extends PixelMenu
class_name InspectLibraryScene

@export var song_display: SongDisplayPanel
@export var radial_selector: RadialSelector

func start_anim() -> void: 
	for child in radial_selector.get_children(): 
		if child is RichTextLabel: child.queue_free()
	for track in Global.menu_manager.music_tagger_node.get_all_tracks():
		var label = RichTextLabel.new()
		label.custom_minimum_size = Vector2(1000, 100)
		label.text = track.track_title
		radial_selector.add_child(label)
func end_anim() -> void: pass

func _process(_delta: float) -> void:
	var track_idx = radial_selector.get_closest_idx()
	var track = Global.menu_manager.music_tagger_node.get_all_tracks()[track_idx]
	song_display.display_track(track)
