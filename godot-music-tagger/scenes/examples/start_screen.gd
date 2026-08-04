extends PixelMenu
class_name StartScreen

@export var buttons: Array[DefaultButton]
var all_t : Array[Tweenable] = []
var t: Tween 

func _ready() -> void: 
	all_t = get_all_tweenables(self)
	for but in buttons:
		but.pressed.connect(_on_button_pressed.bind(but.name))

func _on_button_pressed(_name:String) -> void:
	match _name.to_lower():
		"setup":
			Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.SETUP))
		"library":
			var result = Global.menu_manager.music_tagger_node.try_load_cache()
			if result:
				Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.INSPECT))
			else:
				Global.menu_manager.transition_to_scene(SceneDatabase.get_scene(SceneDatabase.Scene.SETUP))
		"quit":
			pass
		_:
			push_warning("PixelMenu(%s) failed to find button name <%s>" % [self, _name])

func start_anim() -> void: 
	show()
func end_anim() -> void: 
	hide()
