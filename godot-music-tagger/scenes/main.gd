extends PixelMenuManager
class_name MusicManager

@onready var music_tagger_node: MusicTaggerNode = $MusicTaggerNode

func _ready() -> void:
	super()
	Global.menu_manager = self
	SignalBus.scan.connect(_on_scan)

func _on_scan(dirs: Array[String]) -> void:
	if dirs.size() == 0: return
	music_tagger_node.scan_directory(dirs[0])
