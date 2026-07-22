extends PixelMenu
class_name InspectLibraryScene

signal possible_genres(genres:Array[String])
signal possible_tags(tags:Array[String])
@export var song_display: SongDisplayPanel
@export var radial_selector: RadialSelector
@export var search_man: SearchManager
var _genres: Array[String] = []
var _tags: Array[String] = []
var _idxs: Array[int] = []
var _selected_idx: int = 0

func _enter_tree() -> void:
	radial_selector.selected_item_changed.connect(func(idx:int): 
		_selected_idx = idx
		call_deferred("_on_selection_changed")
	)
	search_man.search_query_changed.connect(_on_search)
	if Global.menu_manager.music_tagger_node.has_library():
		_set_all_tracks(Global.menu_manager.music_tagger_node.get_all_tracks())
	else:
		Global.menu_manager.music_tagger_node.library_scanned.connect(func():
			_set_all_tracks(Global.menu_manager.music_tagger_node.get_all_tracks())
		)
	

func start_anim() -> void: 
	pass

func end_anim() -> void: pass

func _set_all_tracks(all_tracks: Array[GodotTrack]):
	for child in radial_selector.get_children(): 
		if child is RichTextLabel: child.queue_free()
	#await get_tree().process_frame
	for track in all_tracks:
		var label = RichTextLabel.new()
		label.custom_minimum_size = Vector2(1000, 100)
		label.text = track.track_title
		label.mouse_filter = Control.MOUSE_FILTER_PASS
		label.visible = false
		radial_selector.add_child(label)
	_genres = Global.menu_manager.music_tagger_node.get_all_genres()
	_tags = Global.menu_manager.music_tagger_node.get_all_custom_tags()
	#song_display.set_genres(_genres)
	possible_genres.emit(_genres)
	possible_tags.emit(_tags)
	#song_display.set_possible_tags(_tags)
	
	

func _on_search(query:String, selected_tags:Array[String], selected_genres:Array[String]) -> void:
	var searched_tracks = Global.menu_manager.music_tagger_node.search_tracks(query, selected_tags, selected_genres)
	_idxs = Global.menu_manager.music_tagger_node.searched_track_idxs
	radial_selector.scroll_angle = 0.
	radial_selector.target_scroll_angle = 0.
	print("Query sent")
	_set_all_tracks(searched_tracks)

func _on_selection_changed() -> void:
	var track_idx = _selected_idx if _idxs.size() == 0 else _idxs[clampi(_selected_idx, 0, _idxs.size()-1)]
	if track_idx == -1: return
	var track = Global.menu_manager.music_tagger_node.get_track_at(track_idx)
	if not track: return
	song_display.display_track(track)
