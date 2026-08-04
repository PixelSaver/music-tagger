extends Control


func _ready() -> void:
	get_viewport().size_changed.connect(_update_size)
	_update_size()
func _update_size():
	$ColorRect.size = Vector2(get_viewport_rect().size.x*1.2 - 350, 120)
	#self.custom_minimum_size = Vector2(get_viewport_rect().size.x*1.2 - 400, 100)
