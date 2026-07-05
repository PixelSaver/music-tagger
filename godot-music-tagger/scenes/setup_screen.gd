extends PixelMenu
class_name SetupScreen

@onready var mt : MusicTaggerNode 
@export var logs: RichTextLabel
@export var dir_input: LineEdit
@export var confirm_button: DefaultButton
@export var next_button: DefaultButton

func _ready() -> void:
	await get_tree().process_frame
	mt = Global.menu_manager.music_tagger_node
	mt.error.connect(_status)
	mt.scan_progress.connect(_status)
	mt.track_found.connect(_status)
	confirm_button.pressed.connect(_on_scan_but_pressed)
	next_button.pressed.connect(_on_next_but_pressed)

func _on_next_but_pressed() -> void:
	Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.INSPECT))
func _on_scan_but_pressed() -> void:
	var dirs: Array[String] = []
	dirs.append(dir_input.text)
	SignalBus.scan.emit(dirs)
func _status(_input: String) -> void:
	pass

func start_anim() -> void: pass
func end_anim() -> void: queue_free()
