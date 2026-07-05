extends PixelMenu
class_name StartMenu

@export var buttons: Array[DefaultButton]
var all_t : Array[Tweenable] = []
var t: Tween 


func start_anim() -> void: pass
func end_anim() -> void: pass
