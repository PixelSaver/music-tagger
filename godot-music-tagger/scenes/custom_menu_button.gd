@tool
extends Button
class_name CustomMenuButton

signal idx_pressed(idx:int)
var items: Array[CustomMenuButtonItem] = []
var popup: Control
var popup_cont: Control

#region Making the popup
func _enter_tree() -> void:
	popup = get_node_or_null("Popup")
	popup_cont = get_node_or_null("Popup/MarginContainer/ScrollContainer/VBoxContainer")
	if popup != null or not Engine.is_editor_hint(): return
	var _owner = get_tree().edited_scene_root
	popup = Panel.new()
	popup.name = "Popup"
	popup.tree_exiting.connect(func(): popup = null)
	add_child(popup)
	popup.position = self.get_rect().size * Vector2(0., 1.)
	popup.owner = _owner
	popup.custom_minimum_size = Vector2(100, 10)
	
	var mcont := MarginContainer.new()
	popup.add_child(mcont)
	mcont.name = "MarginContainer"
	mcont.set_anchors_preset(Control.PRESET_FULL_RECT)
	mcont.owner = _owner
	
	var scroll := ScrollContainer.new()
	mcont.add_child(scroll)
	scroll.name = "ScrollContainer"
	scroll.owner = _owner
	
	var vbox = VBoxContainer.new()
	scroll.add_child(vbox)
	vbox.name = "VBoxContainer"
	vbox.owner = _owner
	popup_cont = vbox

func _ready() -> void:
	popup.hide()

func _pressed() -> void:
	popup.visible = !popup.visible

func _notification(what: int) -> void:
	match what:
		NOTIFICATION_RESIZED:
			print("Resized")
			_update_popup_position()
#endregion

func _update_popup_position() -> void:
	popup.top_level = true
	popup.global_position = global_position + self.get_rect().size * Vector2(0., 1.)

func _update_popup_items() -> void:
	for child in popup_cont.get_children():
		child.queue_free()
	for item in items:
		var button = CheckBox.new() if item.checkable else Button.new()
		button.pressed.connect(func(): self.idx_pressed.emit(item.idx))
		button.custom_minimum_size = Vector2(0., 50)
		popup_cont.add_child(button)
		button.text = item.text
		#var label = Label.new()
		#label.text = item.text
		#button.add_child(label)
		#label.set_anchors_preset(Control.PRESET_FULL_RECT)
	var desired = popup_cont.get_combined_minimum_size()
	popup.size = Vector2(desired.x, min(desired.y, 300))

#region Utility functions exposing items
func clear_popup() -> void:
	items.clear()

func get_item_text(id:int) -> String:
	return items[id].text

func set_item_checked(id:int, checked:bool) -> void:
	if id >= items.size() or id < 0: return
	var item = items[id]
	item.checked = (item.checkable && checked)
	call_deferred("_update_popup_items")

func add_popup_item(_text:String, _idx:int=-1):
	if _idx == -1: _idx = items.size()
	items.append(CustomMenuButtonItem.new(_text, _idx, false))
	call_deferred("_update_popup_items")
func add_checkable_popup_item(_text:String, _idx:int=-1):
	if _idx == -1: _idx = items.size()
	items.append(CustomMenuButtonItem.new(_text, _idx, true))
	call_deferred("_update_popup_items")
#endregion

class CustomMenuButtonItem extends Resource: 
	@export var idx: int
	@export var text: String
	@export var checkable: bool
	@export var checked: bool = false
	
	func _init(_text:String, _idx:int, _checkable:bool) -> void:
		idx = _idx
		text = _text
		checkable = _checkable
