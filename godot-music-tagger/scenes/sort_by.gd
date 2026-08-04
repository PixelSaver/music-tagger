extends HBoxContainer
class_name SortByContainer

enum SortMethod {
	RELEVANT,
	ALPHABETIC_ASC,
	ALPHABETIC_DESC,
}
signal method_requested(method:SortMethod)
@export var option_button : OptionButton

func _ready() -> void:
	_setup_options()
	option_button.get_popup().add_theme_font_size_override("font_size", 18)
	option_button.item_selected.connect(_on_idx_pressed)

func _setup_options() -> void:
	option_button.clear()
	for key in SortMethod.keys():
		var label = key as String
		option_button.add_item(label.capitalize())

func _on_idx_pressed(idx:int) -> void:
	method_requested.emit(idx)
