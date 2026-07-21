extends VBoxContainer
class_name SearchManager

@export var search_bar: LineEdit
@export var tags_filter: SearchFilter
@export var genres_filter: SearchFilter

signal search_query_changed(query:String, tags:Array[String], genres:Array[String])
func _ready() -> void:
	search_bar.text_changed.connect(func(_t:String): _emit_query())
	tags_filter.search_filters_changed.connect(func(_n:Array[String]): _emit_query())
	genres_filter.search_filters_changed.connect(func(_n:Array[String]): _emit_query())

func _emit_query() -> void:
	print("queried")
	var q := search_bar.text
	var t := tags_filter.get_selected_filters()
	var g := genres_filter.get_selected_filters()
	search_query_changed.emit(q, t, g)


func _on_possible_genres(genres: Array[String]) -> void:
	genres_filter.set_possible_filters(genres)


func _on_possible_tags(tags: Array[String]) -> void:
	tags_filter.set_possible_filters(tags)
