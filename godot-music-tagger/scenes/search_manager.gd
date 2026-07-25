extends VBoxContainer
class_name SearchManager

const METHOD = SortByContainer.SortMethod

signal method_requested(method:METHOD)
@export var search_bar: LineEdit
@export var tags_filter: SearchFilter
@export var genres_filter: SearchFilter
@export var sort_by: SortByContainer
@onready var dupe_test: CheckBox = $HBoxContainer/DupeTest

signal search_query_changed(query:String, tags:Array[String], genres:Array[String], dupes:bool)
func _ready() -> void:
	search_bar.text_changed.connect(func(_t:String): _emit_query())
	tags_filter.search_filters_changed.connect(func(_n:Array[String]): _emit_query())
	genres_filter.search_filters_changed.connect(func(_n:Array[String]): _emit_query())
	sort_by.method_requested.connect(_on_method_req)
	dupe_test.toggled.connect(func(_n:bool): _emit_query())


func _on_method_req(method:METHOD) -> void:
	method_requested.emit(method)

func _emit_query() -> void:
	print("queried")
	var q := search_bar.text
	var t := tags_filter.get_selected_filters()
	var g := genres_filter.get_selected_filters()
	var d := dupe_test.button_pressed
	search_query_changed.emit(q, t, g, d)


func _on_possible_genres(genres: Array[String]) -> void:
	genres_filter.set_possible_filters(genres)


func _on_possible_tags(tags: Array[String]) -> void:
	tags_filter.set_possible_filters(tags)
