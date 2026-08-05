extends TextureButton
class_name XButton

@export var normal_color: Color
@export var hover_color: Color
@export var pressed_color: Color

func _ready() -> void:
	mouse_entered.connect(_hover)
	mouse_exited.connect(_unhover)
	button_down.connect(_pressed)
	button_up.connect(_released)
	_released()

func _pressed() -> void:
	self.modulate = pressed_color

func _released() -> void:
	if is_hovered():
		modulate = hover_color
	else:
		modulate = normal_color

func _hover() -> void:
	self.modulate = hover_color

func _unhover() -> void:
	print("Unhovered")
	self.modulate = normal_color
