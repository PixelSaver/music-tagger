extends HBoxContainer
class_name CustomTagsDisplay

@export var display_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit
signal tags_changed(tags:Array[String])
var tags : Dictionary[int, Dictionary]= {
	
}

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)

func set_tags(tags:Array[String]) -> void:
	var out = ""
	for tag in tags:
		out += tag
		if tag != tags.back(): out += ", "
	line_edit.text = out
	

func set_possible_tags(_tags: Array[String]) -> void:
	button.get_popup().clear()
	for tag in _tags:
		button.get_popup().add_check_item(tag)
	for idx in button.get_popup().item_count:
		tags.set(button.get_popup().get_item_id(idx), button.get_popup().get_item_text(idx))
		

func _on_idx_pressed(idx:int) -> void:
	var text = button.get_popup().get_item_text(idx)
	print("Id pressed: %s, %s" % [idx, text])
	#genre_picked.emit(text)
	set_tags(text)
