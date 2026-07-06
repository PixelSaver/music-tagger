extends HBoxContainer
class_name Genre

@export var genre_text: RichTextLabel
@export var button: MenuButton
@export var line_edit: LineEdit

func _ready() -> void:
	button.get_popup().id_pressed.connect(_on_id_pressed)
	button.get_popup().add_theme_font_size_override("Normal Font Size", 22)

func set_genre(genre_name:String) -> void:
	line_edit.text = genre_name

func set_genres(genres: Array[String]) -> void:
	button.get_popup().clear()
	for genre in genres:
		button.get_popup().add_item(genre)

func _on_id_pressed(id:int) -> void:
	var text = button.get_popup().get_item_text(id)
	set_genre(text)
