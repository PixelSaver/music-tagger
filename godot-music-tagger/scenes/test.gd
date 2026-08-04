extends Control

@onready var music_tagger: MusicTaggerNode = $MusicTaggerNode
@onready var directory_input: LineEdit = $VBoxContainer/DirectoryInput
@onready var button: DefaultButton = $VBoxContainer/DefaultButton
@onready var status: RichTextLabel = $VBoxContainer/Status

# Called when the node enters the scene tree for the first time.
func _ready() -> void: 
	music_tagger.error.connect(_status)
	music_tagger.scan_progress.connect(_status)
	music_tagger.track_found.connect(_status)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _status(input: String) -> void:
	status.text = status.text + "\n" + input

func _on_default_button_pressed() -> void:
	print(directory_input.text)
	var err = music_tagger.scan_directory(directory_input.text)
	if err.length() > 0:
		printraw(err)
		status.text = err
	else:
		print("Working")
		print(music_tagger.get_all_tracks())
