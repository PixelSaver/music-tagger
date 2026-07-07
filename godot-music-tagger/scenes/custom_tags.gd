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
	button.get_popup().hide_on_checkable_item_selection = false
	button.pressed.connect(func():
		line_edit.grab_focus()
	)
	line_edit.text_submitted.connect(func(new_text:String):
		add_tag(new_text)
	)
func _get_checked_tags() -> Array[String]:
	var out : Array[String] = []
	for key in tags.keys():
		var tag = tags.get(key)
		if tag.checked: out.push_back(tag.name)
	return out
func add_tag(tag_name:String) -> void:
	var popup := button.get_popup()
	var id := popup.item_count

	popup.add_check_item(tag_name)
	popup.set_item_checked(id, true)

	tags[id] = {
		"name": tag_name,
		"checked": true,
	}

	set_tags(_get_checked_tags())

func set_tags(_tags:Array[String]) -> void:
	var out = ""
	for tag in _tags:
		out += tag
		if tag != _tags.back(): out += ", "
	for key in tags.keys():
		var n = tags.get(key).name 
		var found = _tags.find(n)
		tags.set(key, {
			"name": n,
			"checked": found
		})
	tags_changed.emit(_tags)
	line_edit.text = out
	

func set_possible_tags(_tags: Array[String]) -> void:
	button.get_popup().clear()
	for tag in _tags:
		button.get_popup().add_check_item(tag)
	for idx in button.get_popup().item_count:
		tags.set(button.get_popup().get_item_id(idx), {
			"name": button.get_popup().get_item_text(idx),
			"checked": false
		})
		
func _toggle_idx(idx:int) -> void:
	var dict = tags.get(idx)
	var checked = dict.checked
	tags.set(idx, {
		"name": dict.name,
		"checked": !checked
	})
	button.get_popup().set_item_checked(idx, !checked)
	set_tags(_get_checked_tags())

func _on_idx_pressed(idx:int) -> void:
	var text = button.get_popup().get_item_text(idx)
	print("Id pressed: %s, %s" % [idx, text])
	#genre_picked.emit(text)
	_toggle_idx(idx)
	#set_tags()
