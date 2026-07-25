extends HBoxContainer
class_name CustomFixesDisplay

@export var button: MenuButton
@export var line_edit: LineEdit
signal fixes_changed(fixes:Array[String])
var possible_fixes: Array[String] = []
var selected_fixes: Array[String] = []
var searched_fixes: Array[String] = []

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
			_refresh_popup(possible_fixes)
		else:
			searched_fixes = MusicTaggerNode.search_list(possible_fixes, new_text)
			_refresh_popup(searched_fixes)
	)
	line_edit.text_submitted.connect(func(new_text:String):
		add_fix(new_text)
		pass
	)
	self.set_possible_fixes([
		"Lyrics",
		"Song",
		"Artist",
		"CoverArt",
	    "Removal"
	])

func _refresh_popup(fixes:Array[String]) -> void:
	var pop = button.get_popup()
	pop.clear()
	for fixe in fixes:
		pop.add_check_item(fixe)
		pop.set_item_checked(pop.item_count - 1, selected_fixes.has(fixe))

## Sets possible fixes, selected fixes, and updates popup options
func set_possible_fixes(_possible_fixes: Array[String]) -> void:
	possible_fixes = _possible_fixes
	var pop := button.get_popup()
	pop.clear()
	
	for fixe in _possible_fixes:
		pop.add_check_item(fixe)
		pop.set_item_checked(
			pop.item_count-1,
			selected_fixes.has(fixe)
		)
	_update_display()
func set_selected_fixes(fixes:Array[String]) -> void:
	selected_fixes = fixes
	var pop := button.get_popup()
	for i in pop.item_count:
		var n = pop.get_item_text(i)
		pop.set_item_checked(i, selected_fixes.has(n))
	_update_display()
func _update_display() -> void:
	line_edit.text = ", ".join(selected_fixes)
func _on_idx_pressed(idx:int) -> void:
	var pop := button.get_popup()
	var fixe = pop.get_item_text(idx)
	if selected_fixes.has(fixe):
		selected_fixes.erase(fixe)
	else:
		selected_fixes.append(fixe)
	pop.set_item_checked(idx, selected_fixes.has(fixe))
	_update_display()
	fixes_changed.emit(selected_fixes)
func add_fix(fix:String) -> void:
	if possible_fixes.has(fix):
		if selected_fixes.has(fix): return
		selected_fixes.append(fix)
		return
	possible_fixes.append(fix)
	selected_fixes.append(fix)
	set_possible_fixes(possible_fixes)
	_update_display()
	fixes_changed.emit(selected_fixes)
