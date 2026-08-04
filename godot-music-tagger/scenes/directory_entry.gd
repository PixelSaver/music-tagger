extends HBoxContainer
class_name DirectoryEntry

signal text_changed(new_text: String)
signal delete_pressed
@onready var line_edit: LineEdit = $LineEdit
@onready var delete_button: DefaultButton = $DeleteButton

func _ready() -> void:
	line_edit.text_changed.connect(func(new_text:String): text_changed.emit(new_text))
	delete_button.pressed.connect(func(): delete_pressed.emit())

func set_dir(dir:String) -> void:
	if not line_edit: await self.ready
	line_edit.text = dir
func get_dir() -> String:
	return line_edit.text.strip_edges()
