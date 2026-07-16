extends Panel
class_name SongDisplayPanel

@export var title: RichTextLabel
@export var cover: TextureRect
@export var desc: RichTextLabel
@export var genre: Genre
@export var tags: CustomTagsDisplay
var genres: Array[String] = []
var _track: GodotTrack = null
var _cover_texture : ImageTexture

func _ready() -> void:
	_cover_texture = ImageTexture.new()
	cover.texture = _cover_texture
	genre.genre_picked.connect(func(_genre:String):
		Global.menu_manager.music_tagger_node.find_track_write_genre(_track.isrc, _genre)
	)
	tags.tags_changed.connect(func(_tags:Array[String]):
		print("Tags changed:", _tags)
		_track.custom_tags = _tags
		Global.menu_manager.music_tagger_node.find_track_write_custom_tags(_track.isrc, _tags)
	)
	Global.menu_manager.music_tagger_node.loaded_cover_art.connect(_on_cover_art_received)

func set_genres(_genres: Array[String]) -> void:
	genre.set_possible_genres(_genres)
	genres = _genres
func set_possible_tags(possible_tags: Array[String]) -> void:
	tags.set_possible_tags(possible_tags)

func display_track(track:GodotTrack) -> void:
	if track == _track: return
	_track = track
	title.text = track.track_title
	desc.text = ""
	if track.track_artist.length() > 0:
		desc.text += "Artist: %s\n" % track.track_artist
	if track.composer.length() > 0:
		desc.text += "Composer: %s\n" % track.composer
	#if track.genre.length() > 0:
		#desc.text += "Genre: %s\n" % track.genre
	#if track.custom_tags.size() > 0:
		#desc.text += "Custom Tags: "
		#for tag in track.custom_tags:
			#if tag.length() <= 0: continue
			#desc.text += "%s," % tag
		#desc.text += "\n"
	if track.cover_art != null:
		cover.texture = ImageTexture.create_from_image(track.cover_art)
	else:
		var im = _get_cover_art()
		if im != null: 
			print("Trying to get cover art worked!! %s" % str(im))
			cover.texture = ImageTexture.create_from_image(im)
			return
	if track.genre.length() > 0:
		genre.set_genre(track.genre)
	
	tags.set_selected_tags(track.custom_tags)

func _get_cover_art():
	Global.menu_manager.music_tagger_node.request_track_cover_art(_track.isrc)
func _on_cover_art_received(isrc:String, image:Image) -> void:
	if isrc != _track.isrc: return
	_cover_texture.set_image(image)
	
