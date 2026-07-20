extends HBoxContainer
class_name CustomTagsDisplay

@export var button: MenuButton
@export var line_edit: LineEdit
signal tags_changed(tags:Array[String])
var possible_tags: Array[String] = []
var selected_tags: Array[String] = []
var searched_tags: Array[String] = []

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)
	button.get_popup().hide_on_checkable_item_selection = false
	button.pressed.connect(func():
		line_edit.release_focus()
		line_edit.grab_focus()
	)
	line_edit.editing_toggled.connect(func(toggled_on:bool):
		if toggled_on: line_edit.text = ""
	)
	line_edit.text_changed.connect(func(new_text:String):
		if new_text.is_empty():
			_refresh_popup(possible_tags)
		else:
			searched_tags = MusicTaggerNode.search_list(possible_tags, new_text)
			_refresh_popup(searched_tags)
	)
	line_edit.text_submitted.connect(func(new_text:String):
		add_tag(new_text)
		pass
	)

func _refresh_popup(tags:Array[String]) -> void:
	var pop = button.get_popup()
	pop.clear()
	for tag in tags:
		pop.add_check_item(tag)
		pop.set_item_checked(pop.item_count - 1, selected_tags.has(tag))

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
