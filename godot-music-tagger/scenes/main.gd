extends PixelMenuManager
class_name MusicManager

@onready var music_tagger_node: MusicTaggerNode = $MusicTaggerNode
@export var settings := TaggerSettings.new()

func _ready() -> void:
	Global.menu_manager = self
	load_settings(settings)
	if music_tagger_node.try_load_cache() == false:
		music_tagger_node.scan_directory(settings.music_directories[0])
	super()


func load_settings(s:TaggerSettings):
	music_tagger_node.cache_directory = s.cache_directory
	music_tagger_node.music_directories = s.music_directories
	music_tagger_node.playlist_directory = s.playlist_directory
