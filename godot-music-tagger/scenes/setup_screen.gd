extends PixelMenu
class_name SetupScreen
const DIRECTORY_ENTRY = preload("res://scenes/directory_entry.tscn")

@onready var mt : MusicTaggerNode 
@export var music_directory_cont: VBoxContainer 
@export var add_dir_but: DefaultButton
@export var cache_dir: LineEdit
@export var scan_cache_button: DefaultButton
@export var force_scan_button: DefaultButton
@export var explore_library_button: DefaultButton
var dir_entries : Array[DirectoryEntry] = []
var _total_items := -1
var _processed_items := -1
var _scan_notif: PixelNotification

func _ready() -> void:
	await get_tree().process_frame
	mt = Global.menu_manager.music_tagger_node
	#mt.error.connect(_status)
	#mt.scan_progress.connect(_status)
	#mt.track_found.connect(_status)
	scan_cache_button.pressed.connect(_on_scan_but_pressed)
	force_scan_button.pressed.connect(_on_force_scan_but_pressed)
	explore_library_button.pressed.connect(_on_explore_library_pressed)
	add_dir_but.pressed.connect(_add_dir)
	cache_dir.placeholder_text = Global.menu_manager.music_tagger_node.cache_directory
	cache_dir.text_submitted.connect(_on_cache_submit)
	mt.scan_began.connect(_on_scan_begin)
	mt.scan_tick.connect(_on_scan_tick)
	#cache_dir.focus_entered.connect(_on_cache_focused)
	#cache_dir.focus_exited.connect(_on_cache_unfocused)
	
func _on_scan_begin(total_items: int) -> void:
	_total_items = total_items
	_scan_notif = Global.notif_manager.create_notification("Scanning directories", "Reading the tags and cover art of music in directories specified.", -1, true)
func _on_scan_tick(finished_items: int) -> void:
	if not _scan_notif: 
		push_warning("No scan notification when ticking")
		return
	_processed_items = finished_items
	_update_scan_progress(finished_items, _total_items)

func _update_scan_progress(val: int, max_val: int) -> void:
	print("New progress: %s/%s | %s" % [str(val), str(max_val), str(float(val)/float(max_val))])
	if not _scan_notif: 
		push_warning("No scan notification when updating progress")
		return
	_scan_notif.set_progress(float(val), float(max_val))
	if val >= max_val:
		_scan_notif.end_anim()
		_scan_notif = null
	

#region Button reactions
func _on_explore_library_pressed() -> void:
	Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.INSPECT))

func _on_force_scan_but_pressed() -> void:
	var dirs: Array[String] = []
	for child in music_directory_cont.get_children():
		var dir = child as DirectoryEntry
		dirs.append(dir.get_dir())
		dir.tree_exiting.connect(func():
			dir_entries.erase(dir)
		)
	#SignalBus.scan.emit()
	var _result = _force_scan()
	#if !result:
		#print("Awaiting")
		#await Global.menu_manager.music_tagger_node.library_scanned
		#print("Fnished")
	
	

func _on_scan_but_pressed() -> void:
	var dirs: Array[String] = []
	for child in music_directory_cont.get_children():
		var dir = child as DirectoryEntry
		dirs.append(dir.get_dir())
		dir.tree_exiting.connect(func():
			dir_entries.erase(dir)
		)
	#SignalBus.scan.emit()
	var _result = _try_cache_or_scan()
	#if !result:
		#print("Awaiting")
		#await Global.menu_manager.music_tagger_node.library_scanned
		#print("Fnished")
	#Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.INSPECT))
#endregion
#region Music directory functions
func _delete_dir_entry(dir:DirectoryEntry) -> void:
	var children = music_directory_cont.get_children()
	if children.size() == 1: 
		_add_dir()
	for child in children:
		if child == dir:
			dir.queue_free()
			return
func _setup_dirs() -> void:
	var dirs = Global.menu_manager.music_tagger_node.music_directories
	for child in music_directory_cont.get_children(): child.queue_free()
	for i in range(dirs.size()):
		_add_dir(dirs[i])
func _update_dir(dir:String, idx:int) -> void:
	var m_dirs = Global.menu_manager.music_tagger_node.music_directories
	if m_dirs.size() <= idx or idx < 0: 
		push_warning("Update dir idx is outside of Music Directories on MusicTaggerNode: idx=%s" % idx)
	m_dirs[idx] = dir
func _add_dir(dir:String="") -> void:
	var inst = DIRECTORY_ENTRY.instantiate() as DirectoryEntry
	inst.set_dir(dir)
	inst.text_changed.connect(_update_dir.bind(dir_entries.size()))
	inst.delete_pressed.connect(_delete_dir_entry.bind(inst))
	dir_entries.append(inst)
	
	music_directory_cont.add_child(inst)
#endregion

#region Cache directory functions
func _on_cache_submit(text:String) -> void:
	Global.menu_manager.music_tagger_node.cache_directory = text
#func _on_cache_focused() -> void:
	#cache_dir.clear()
#func _on_cache_unfocused() -> void:
	#cache_dir.placeholder_text = Global.menu_manager.music_tagger_node.cache_directory
func _try_cache_or_scan() -> bool:
	var result = Global.menu_manager.music_tagger_node.try_load_cache()
	print("Cached result: %s" % result)
	for dir in Global.menu_manager.music_tagger_node.music_directories:
		print("Music dir: %s" % dir)
	if result == false:
		print("Scan result: %s" % Global.menu_manager.music_tagger_node.scan_directory(Global.menu_manager.music_tagger_node.music_directories[0]))
	Global.menu_manager.has_library = true
	return result
func _force_scan() -> bool:
	for dir in Global.menu_manager.music_tagger_node.music_directories:
		print("Music dir: %s" % dir)
	var result = Global.menu_manager.music_tagger_node.scan_directory(Global.menu_manager.music_tagger_node.music_directories[0])
	if result.length() > 0: print("Scan result: %s" % result)
	Global.menu_manager.has_library = true
	return not result.is_empty()
#endregion


func start_anim() -> void: 
	_setup_dirs()
func end_anim() -> void: 
	hide()
