extends HBoxContainer
class_name SortByContainer

enum SortMethod {
	RELEVANT,
	ALPHABETIC_ASC,
	ALPHABETIC_DESC,
}
signal method_requested(method:SortMethod)
var current_method := SortMethod.RELEVANT
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

func select_method(new_method: SortMethod) -> void:
	if new_method == current_method: return
	option_button.select(new_method)
	_on_idx_pressed(new_method)

func _on_idx_pressed(idx:int) -> void:
	print("New method req")
	current_method = idx as SortMethod
	method_requested.emit(idx)
