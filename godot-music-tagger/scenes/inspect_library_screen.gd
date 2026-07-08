extends PixelMenu
class_name InspectLibraryScene

@export var song_display: SongDisplayPanel
@export var radial_selector: RadialSelector
@export var search_bar: LineEdit

func _ready() -> void:
	radial_selector.selected_item_changed.connect(_on_selection_changed)
	search_bar.text_changed.connect(_on_search)

func start_anim() -> void: 
	_set_all_tracks(Global.menu_manager.music_tagger_node.get_all_tracks())

func end_anim() -> void: pass

func _set_all_tracks(all_tracks: Array[GodotTrack]):
	for child in radial_selector.get_children(): 
		if child is RichTextLabel: child.queue_free()
	#await get_tree().process_frame
	for track in all_tracks:
		var label = RichTextLabel.new()
		label.custom_minimum_size = Vector2(1000, 100)
		label.text = track.track_title
		radial_selector.add_child(label)

func _on_search(text:String) -> void:
	if text.is_empty(): 
		_set_all_tracks(Global.menu_manager.music_tagger_node.get_all_tracks())
	else:
		var searched_tracks = Global.menu_manager.music_tagger_node.search_tracks(text)
		
		_set_all_tracks(searched_tracks)

func _on_selection_changed(track_idx:int) -> void:
	var track = Global.menu_manager.music_tagger_node.get_all_tracks()[track_idx]
	var genres = Global.menu_manager.music_tagger_node.get_all_genres()
	if genres.size() > 0: song_display.set_genres(genres)
	var tags = Global.menu_manager.music_tagger_node.get_all_custom_tags()
	song_display.set_possible_tags(tags)
	song_display.display_track(track)
