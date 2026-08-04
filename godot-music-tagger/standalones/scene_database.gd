extends Node

const DEFAULT_SCENE = Scene.START
enum Scene {
	START,
	SETUP,
	INSPECT,
}

var scenes = {
	Scene.START: preload("res://scenes/examples/start_screen.tscn"),
	Scene.SETUP: preload("res://scenes/setup_screen.tscn"),
	Scene.INSPECT: preload("res://scenes/inspect_library_screen.tscn"),
}
func get_scene(scene:Scene) -> PackedScene:
	if scenes.has(scene):
		return scenes.get(scene)
	else: 
		print("Failed to get scene #(%s), returning scene#(%s)" % [scene, DEFAULT_SCENE])
		return scenes.get(DEFAULT_SCENE)
