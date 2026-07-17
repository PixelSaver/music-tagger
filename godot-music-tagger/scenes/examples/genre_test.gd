extends HBoxContainer
#class_name Genre

@export var genre_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit
var possible_genres : Array[String] = []
var selected_genres : Array[String] = []
var searched_genres : Array[String] = []
signal genres_changed(genres:Array[String])

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
			_refresh_popup(possible_genres)
		else:
			searched_genres = MusicTaggerNode.search_list(possible_genres, new_text)
			_refresh_popup(searched_genres)
	)
	line_edit.text_submitted.connect(func(new_text:String):
		add_tag(new_text)
		pass
	)

func _refresh_popup(genres:Array[String]) -> void:
	var pop = button.get_popup()
	pop.clear()
	for g in genres:
		pop.add_check_item(g)
		pop.set_item_checked(pop.item_count - 1, selected_genres.has(g))

func set_genre(genre_name:String) -> void:
	line_edit.text = genre_name
	
## Sets possible tags, selected tags, and updates popup options
func set_possible_genres(_possible_genres: Array[String]) -> void:
	possible_genres = _possible_genres
	var pop := button.get_popup()
	pop.clear()
	
	for tag in _possible_genres:
		pop.add_check_item(tag)
		pop.set_item_checked(
			pop.item_count-1,
			selected_genres.has(tag)
		)
	_update_display()
func set_selected_genres(genres:Array[String]) -> void:
	selected_genres = genres
	var pop := button.get_popup()
	for i in pop.item_count:
		var n = pop.get_item_text(i)
		pop.set_item_checked(i, selected_genres.has(n))
	_update_display()
func _update_display() -> void:
	line_edit.text = ", ".join(selected_genres)
func add_tag(genre:String) -> void:
	if possible_genres.has(genre):
		if selected_genres.has(genre): return
		selected_genres.append(genre)
		return
	possible_genres.append(genre)
	selected_genres.append(genre)
	set_possible_genres(possible_genres)
	_update_display()
	genres_changed.emit(selected_genres)

func _on_idx_pressed(idx:int) -> void:
	var pop := button.get_popup()
	var genre = pop.get_item_text(idx)
	if selected_genres.has(genre):
		selected_genres.erase(genre)
	else:
		selected_genres.append(genre)
	pop.set_item_checked(idx, selected_genres.has(genre))
	_update_display()
	genres_changed.emit(selected_genres)
