extends HBoxContainer
class_name SearchFilter

@export var button: MenuButton
@export var line_edit: LineEdit
signal search_filters_changed(new_filters:Array[String])
var possible_filters: Array[String] = []
var selected_filters: Array[String] = []
var searched_filters: Array[String] = []

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)
	button.get_popup().hide_on_checkable_item_selection = false
	button.get_popup().add_theme_font_size_override("font_size", 18)
	#button.pressed.connect(func():
		#line_edit.release_focus()
		#line_edit.grab_focus()
	#)
	line_edit.editing_toggled.connect(func(toggled_on:bool):
		if toggled_on: line_edit.text = ""
	)
	line_edit.text_changed.connect(func(new_text:String):
		if new_text.is_empty():
			_refresh_popup(possible_filters)
		else:
			searched_filters = MusicTaggerNode.search_list(possible_filters, new_text)
			_refresh_popup(searched_filters)
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
		pop.set_item_checked(pop.item_count - 1, selected_filters.has(tag))

## Sets possible tags, selected tags, and updates popup options
func set_possible_filters(_possible_filters: Array[String]) -> void:
	possible_filters = _possible_filters
	var pop := button.get_popup()
	pop.clear()
	
	for tag in _possible_filters:
		pop.add_check_item(tag)
		pop.set_item_checked(
			pop.item_count-1,
			selected_filters.has(tag)
		)
	_update_display()
func set_selected_filters(tags:Array[String]) -> void:
	selected_filters = tags
	var pop := button.get_popup()
	for i in pop.item_count:
		var n = pop.get_item_text(i)
		pop.set_item_checked(i, selected_filters.has(n))
	_update_display()
func _update_display() -> void:
	line_edit.text = ", ".join(selected_filters)
func _on_idx_pressed(idx:int) -> void:
	print("New idx printed for %s, %s" % [self.name, idx])
	var pop := button.get_popup()
	var tag = pop.get_item_text(idx)
	if selected_filters.has(tag):
		selected_filters.erase(tag)
	else:
		selected_filters.append(tag)
	pop.set_item_checked(idx, selected_filters.has(tag))
	_update_display()
	search_filters_changed.emit(selected_filters)
#TODO Make sure the search filters are updated with the global tags and stuff
func add_tag(tag:String) -> void:
	if possible_filters.has(tag):
		if selected_filters.has(tag): return
		selected_filters.append(tag)
		return
	possible_filters.append(tag)
	selected_filters.append(tag)
	set_possible_filters(possible_filters)
	_update_display()
	search_filters_changed.emit(selected_filters)

func get_selected_filters() -> Array[String]:
	return selected_filters
