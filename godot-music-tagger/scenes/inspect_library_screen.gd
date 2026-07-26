extends PixelMenu
class_name InspectLibraryScene

const H_TRACK_DISPLAY = preload("res://scenes/h_song_display.tscn")

signal possible_genres(genres:Array[String])
signal possible_tags(tags:Array[String])
@export var song_display: SongDisplayPanel
@export var radial_selector: RadialSelector
@export var search_man: SearchManager
@export var settings_button: DefaultButton
var _displayed_tracks: Array[GodotTrack] = []
var _genres: Array[String] = []
var _tags: Array[String] = []
var _idxs: Array[int] = []
var _selected_idx: int = 0

var _virtual_idx: int = 0
var _pool: Array[HTrackDisplay] = []
var _pool_capacity := 0

func _enter_tree() -> void:
	radial_selector.selected_item_changed.connect(func(idx:int): 
		_selected_idx = idx + _virtual_idx
		call_deferred("_on_selection_changed_and_pool")
	)
	search_man.search_query_changed.connect(_on_search)
	search_man.method_requested.connect(_on_method_req)
	
	settings_button.pressed.connect(func():
		Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.SETUP))
	)
	
	if Global.menu_manager.music_tagger_node.has_library():
		_displayed_tracks = Global.menu_manager.music_tagger_node.get_all_tracks()
		_set_all_tracks(_displayed_tracks)
	else:
		Global.menu_manager.music_tagger_node.library_scanned.connect(func():
			_displayed_tracks = Global.menu_manager.music_tagger_node.get_all_tracks()
			_set_all_tracks(_displayed_tracks)
		)
	

func start_anim() -> void: 
	pass

func end_anim() -> void: 
	queue_free()

func _on_selection_changed_and_pool() -> void:
	_update_pool_for_selected_index()
	_on_selection_changed()

func _on_selection_changed() -> void:
	if _displayed_tracks.size() > 0:
		song_display.display_track(_displayed_tracks[_selected_idx])
	else:
		var track_idx = _selected_idx if _idxs.size() == 0 else _idxs[clampi(_selected_idx, 0, _idxs.size()-1)]
		if track_idx == -1: return
		song_display.display_track(
			Global.menu_manager.music_tagger_node.get_track_at(track_idx)
		)
	#var track = Global.menu_manager.music_tagger_node.get_track_at(track_idx)
	

func _set_all_tracks(all_tracks: Array[GodotTrack]):
	_displayed_tracks = all_tracks
	
	for child in radial_selector.get_children(): 
		if child is RichTextLabel or child is HTrackDisplay: child.queue_free()
	_pool.clear()
	
	# how many pooled items needed
	#var vw: int = radial_selector.visibility_window
	var vw = 1
	_pool_capacity = min(_displayed_tracks.size(), vw*2 + 1)
	if _pool_capacity <= 0: 
		push_warning("Pool capacity is <= 0")
		return
	
	for i in range(_pool_capacity):
		var display := H_TRACK_DISPLAY.instantiate() as HTrackDisplay
		display.visible = false
		_pool.append(display)
		radial_selector.add_child(display)
	
	radial_selector.scroll_to_index(0)
	_update_pool_for_selected_index()
	
	#for track in all_tracks:
		#var label = RichTextLabel.new()
		#label.custom_minimum_size = Vector2(1000, 100)
		#label.text = track.track_title
		#label.mouse_filter = Control.MOUSE_FILTER_PASS
		#label.visible = false
		#radial_selector.add_child(label)
		
		#var display = H_TRACK_DISPLAY.instantiate() as HTrackDisplay
		#display.display_track(track)
		#radial_selector.add_child(display)
	
	_genres = Global.menu_manager.music_tagger_node.get_all_genres()
	_tags = Global.menu_manager.music_tagger_node.get_all_custom_tags()
	radial_selector.scroll_to_index(0)
	#song_display.set_genres(_genres)
	possible_genres.emit(_genres)
	possible_tags.emit(_tags)
	#song_display.set_possible_tags(_tags)

func _update_pool_for_selected_index() -> void:
	if _displayed_tracks.is_empty() or _pool.is_empty():
		push_warning("Displayed tracks or pool was empty")
		return
	
	var idx := radial_selector.get_closest_idx() + _virtual_idx
	if idx < 0: 
		return
	
	#var vw := radial_selector.visibility_window
	var vw: int = 1
	var start_idx := idx - vw
	for i in range(_pool.size()):
		var track_idx : int = start_idx + i
		
		var d := _pool[i]
		if track_idx >= 0 and track_idx < _displayed_tracks.size():
			d.display_track(_displayed_tracks[track_idx])
			#d.visible = true
		#else:
			#d.visible = false

#region Searching 
func _on_method_req(method:SortByContainer.SortMethod) -> void:
	if method == SortByContainer.SortMethod.RELEVANT:
		_displayed_tracks = Global.menu_manager.music_tagger_node.get_all_tracks()
	else:
		_displayed_tracks = MusicTaggerNode.sort_tracks(_displayed_tracks, method)
	_set_all_tracks(_displayed_tracks)

func _on_search(query:String, selected_tags:Array[String], selected_genres:Array[String], dupes:bool) -> void:
	var searched_tracks = Global.menu_manager.music_tagger_node.search_tracks(query, selected_tags, selected_genres, dupes)
	_displayed_tracks = searched_tracks
	_idxs = Global.menu_manager.music_tagger_node.searched_track_idxs
	radial_selector.scroll_angle = 0.
	radial_selector.target_scroll_angle = 0.
	print("Query sent")
	_set_all_tracks(_displayed_tracks)

#endregion
