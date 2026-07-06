extends PixelMenu
class_name InspectLibraryScene

@export var song_display: SongDisplayPanel
@export var radial_selector: RadialSelector

func _ready() -> void:
	radial_selector.selected_item_changed.connect(_on_selection_changed)

func start_anim() -> void: 
	for child in radial_selector.get_children(): 
		if child is RichTextLabel: child.queue_free()
	for track in Global.menu_manager.music_tagger_node.get_all_tracks():
		var label = RichTextLabel.new()
		label.custom_minimum_size = Vector2(1000, 100)
		label.text = track.track_title
		radial_selector.add_child(label)
func end_anim() -> void: pass

func _on_selection_changed(track_idx:int) -> void:
	var track = Global.menu_manager.music_tagger_node.get_all_tracks()[track_idx]
	song_display.display_track(track)
	var genres = Global.menu_manager.music_tagger_node.get_all_genres()
	if genres.size() > 0: song_display.set_genres(genres)
