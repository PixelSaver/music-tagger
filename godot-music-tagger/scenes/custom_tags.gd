extends HBoxContainer
class_name CustomTagsDisplay

@export var display_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit
signal tags_changed(tags:Array[String])
var possible_tags: Array[String] = []
var selected_tags: Array[String] = []

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)
	button.get_popup().hide_on_checkable_item_selection = false
	button.pressed.connect(func():
		line_edit.release_focus()
		line_edit.grab_focus()
	)
	line_edit.text_submitted.connect(func(new_text:String):
		add_tag(new_text)
		pass
	)

## Sets possible tags, selected tags, and updates popup options
func set_possible_tags(_possible_tags: Array[String]) -> void:
	possible_tags = _possible_tags
	var pop := button.get_popup()
	pop.clear()
	
	for tag in _possible_tags:
		pop.add_check_item(tag)
		pop.set_item_checked(
			pop.item_count-1,
			selected_tags.has(tag)
		)
	_update_display()
func set_selected_tags(tags:Array[String]) -> void:
	selected_tags = tags
	var pop := button.get_popup()
	for i in pop.item_count:
		var n = pop.get_item_text(i)
		pop.set_item_checked(i, selected_tags.has(n))
	_update_display()
func _update_display() -> void:
	line_edit.text = ", ".join(selected_tags)
func _on_idx_pressed(idx:int) -> void:
	var pop := button.get_popup()
	var tag = pop.get_item_text(idx)
	if selected_tags.has(tag):
		selected_tags.erase(tag)
	else:
		selected_tags.append(tag)
	pop.set_item_checked(idx, selected_tags.has(tag))
	_update_display()
	tags_changed.emit(selected_tags)
func add_tag(tag:String) -> void:
	if possible_tags.has(tag):
		if selected_tags.has(tag): return
		selected_tags.append(tag)
		return
	possible_tags.append(tag)
	selected_tags.append(tag)
	set_possible_tags(possible_tags)
	_update_display()
	tags_changed.emit(selected_tags)
	#genre_picked.emit(text)
	#_toggle_idx(idx)
	#set_tags()
#func _get_checked_tags() -> Array[String]:
	#var out : Array[String] = []
	#for key in tags.keys():
		#var tag = tags.get(key)
		#if tag.checked: out.push_back(tag.name)
	#return out
#func add_tag(tag_name:String) -> void:
	#var popup := button.get_popup()
	#var id := popup.item_count
#
	#popup.add_check_item(tag_name)
	#popup.set_item_checked(id, true)
#
	#tags[id] = {
		#"name": tag_name,
		#"checked": true,
	#}
#
	#set_tags(_get_checked_tags(), true)
#
#func pick_tags(_tags:Array[String]) -> void:
	#pass
#
#func set_tags(_tags:Array[String], emit:bool=false) -> void:
	#var out = ""
	#for i in range(_tags.size()):
		#var tag = _tags[i]
		#out += tag
		#if i != _tags.size()-1: out += ", "
	#for key in tags.keys():
		#var n = tags.get(key).name 
		#var found = _tags.find(n)
		#tags.set(key, {
			#"name": n,
			#"checked": found
		#})
	#if emit: tags_changed.emit(_tags)
	#line_edit.text = out
	#
#
#func set_possible_tags(_tags: Array[String]) -> void:
	#button.get_popup().clear()
	#for tag in _tags:
		#button.get_popup().add_check_item(tag)
	#for idx in button.get_popup().item_count:
		#tags.set(button.get_popup().get_item_id(idx), {
			#"name": button.get_popup().get_item_text(idx),
			#"checked": false
		#})
		#
#func _toggle_idx(idx:int) -> void:
	#var dict = tags.get(idx)
	#var checked = dict.checked
	#tags.set(idx, {
		#"name": dict.name,
		#"checked": !checked
	#})
	#button.get_popup().set_item_checked(idx, !checked)
	#set_tags(_get_checked_tags(), true)
