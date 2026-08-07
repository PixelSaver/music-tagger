extends PixelMenuManager
class_name MusicManager

@onready var music_tagger_node: MusicTaggerNode = $MusicTaggerNode
@export var settings := TaggerSettings.new()
var has_library := false

func _ready() -> void:
	Global.menu_manager = self
	SignalBus.scan.connect(func():
		for dir in music_tagger_node.music_directories:
			music_tagger_node.scan_directories()
	)
	load_settings(settings)
	
	super()


func load_settings(s:TaggerSettings):
	music_tagger_node.cache_directory = s.cache_directory
	music_tagger_node.music_directories = s.music_directories
	music_tagger_node.playlist_directory = s.playlist_directory
