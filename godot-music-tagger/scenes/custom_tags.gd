extends HBoxContainer
class_name CustomTagsDisplay

@export var display_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit
signal genre_picked(genre:String)

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)

func set_tags(tags:Array[String]) -> void:
	var out = ""
	for tag in tags:
		out += tag
		if tag != tags.back(): out += ", "
	line_edit.text = out
	

func set_possible_tags(genres: Array[String]) -> void:
	button.get_popup().clear()
	for genre in genres:
		button.get_popup().add_item(genre)

func _on_idx_pressed(idx:int) -> void:
	var text = button.get_popup().get_item_text(idx)
	print("Id pressed: %s, %s" % [idx, text])
	genre_picked.emit(text)
	set_tags(text)
