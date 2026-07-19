extends PixelMenu
class_name SetupScreen
const DIRECTORY_ENTRY = preload("res://scenes/directory_entry.tscn")

@onready var mt : MusicTaggerNode 
@export var music_directory_cont: VBoxContainer 
@export var add_dir_but: DefaultButton
@export var cache_dir: LineEdit
@export var scan_button: DefaultButton
var dir_entries : Array[DirectoryEntry] = []

func _ready() -> void:
	await get_tree().process_frame
	mt = Global.menu_manager.music_tagger_node
	#mt.error.connect(_status)
	#mt.scan_progress.connect(_status)
	#mt.track_found.connect(_status)
	scan_button.pressed.connect(_on_scan_but_pressed)
	add_dir_but.pressed.connect(_add_dir)
	cache_dir.placeholder_text = Global.menu_manager.music_tagger_node.cache_directory
	cache_dir.text_submitted.connect(_on_cache_submit)
	#cache_dir.focus_entered.connect(_on_cache_focused)
	#cache_dir.focus_exited.connect(_on_cache_unfocused)

func _on_scan_but_pressed() -> void:
	var dirs: Array[String] = []
	for child in music_directory_cont.get_children():
		var dir = child as DirectoryEntry
		dirs.append(dir.get_dir())
		dir.tree_exiting.connect(func():
			dir_entries.erase(dir)
		)
	SignalBus.scan.emit()
	await Global.menu_manager.music_tagger_node.library_scanned
	Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.INSPECT))
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
func _update_dir(idx:int, dir:String) -> void:
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
#endregion
func start_anim() -> void: 
	_setup_dirs()
func end_anim() -> void: queue_free()
