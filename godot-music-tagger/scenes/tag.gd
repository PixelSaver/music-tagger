extends Label
class_name TagDisplay
@export var panel: Panel 

func set_tag(tag_name: String, tag_color: Color):
	self.text = tag_name
	var box = panel.get_theme_stylebox("panel") as StyleBoxFlat
	box.bg_color = tag_color
