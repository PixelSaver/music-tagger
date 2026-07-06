extends HBoxContainer
class_name Genre

@export var genre_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit

func _ready() -> void:
	button.get_popup().index_pressed.connect(_on_idx_pressed)

func set_genre(genre_name:String) -> void:
	line_edit.text = genre_name

func set_genres(genres: Array[String]) -> void:
	button.get_popup().clear()
	for genre in genres:
		button.get_popup().add_item(genre)

func _on_idx_pressed(idx:int) -> void:
	print("Id pressed: %s" % idx)
	var text = button.get_popup().get_item_text(idx)
	set_genre(text)
