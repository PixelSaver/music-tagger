extends PixelMenuManager
class_name MusicManager

@onready var music_tagger_node: MusicTaggerNode = $MusicTaggerNode
@export var settings := TaggerSettings.new()

func _ready() -> void:
	Global.menu_manager = self
	SignalBus.scan.connect(_on_scan)
	load_settings(settings)
	SignalBus.scan.emit(settings.music_directories)
	super()

func _on_scan(dirs: Array[String]) -> void:
	if dirs.size() == 0: return
	music_tagger_node.scan_directory(dirs[0])

func load_settings(s:TaggerSettings):
	music_tagger_node.cache_directory = s.cache_directory
	music_tagger_node.music_directories = s.music_directories
	music_tagger_node.playlist_directory = s.playlist_directory
