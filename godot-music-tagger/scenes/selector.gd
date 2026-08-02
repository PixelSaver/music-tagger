extends Control


func _ready() -> void:
	get_viewport().size_changed.connect(_update_size)
func _update_size():
	$ColorRect.position.x = -get_viewport_rect().size.x*0.45 + $ColorRect.get_combined_minimum_size().x*.5
