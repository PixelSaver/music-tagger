@tool
extends Container

## Osu-like container to scroll through options
class_name RadialContainer

@export var radius := 100.0:
	set(val):
		radius = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var separation := 10.0:
	set(val):
		separation = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var circle_center := Vector2(100.0, 0):
	set(val):
		circle_center = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var flip := false:
	set(val):
		flip = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var target_scroll_angle := 0.0:
	set(val):
		target_scroll_angle = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var drag_sensitivity := 0.0005
## Proportion taken off the scale of neighboring children.
## Children farther away from current angle are this much smaller
@export var scale_multiplier := 0.1
@export_category("Container Exclusion")
@export var excluded: Array[Node] = []
@export var max_lerp_cooldown := 0.6
var scroll_angle := 0.0
var _lerp_cooldown: float

# Dragging
var _dragging := false
var _last_mouse_pos := Vector2.ZERO


func _ready() -> void:
	self.scroll_angle = 0
	self.target_scroll_angle = self.scroll_angle
	_lerp_cooldown = max_lerp_cooldown


func _notification(what):
	if what == NOTIFICATION_SORT_CHILDREN:
		_update_children()


func _get_layout_children() -> Array[Control]:
	var result: Array[Control] = []
	for child in get_children():
		if not child is Control:
			continue
		if excluded.has(child):
			continue
		result.append(child)
	return result


func _process(delta: float) -> void:
	var children = _get_layout_children()
	if children.size() <= 1:
		return

	var min_limit = -(children.size() - 1) * get_theta()
	var max_limit = 0.0

	var is_overshooting = target_scroll_angle > max_limit or target_scroll_angle < min_limit

	if is_overshooting:
		var target = clampf(target_scroll_angle, min_limit, max_limit)
		target_scroll_angle = lerpf(target_scroll_angle, target, delta * 10.0)

	scroll_angle = lerpf(scroll_angle, target_scroll_angle, delta * 10.0)
	_update_children()

	if Engine.is_editor_hint():
		return
	_lerp_cooldown -= delta
	if _lerp_cooldown < 0.0:
		lerp_to_closest()


## Angle separation between two children
func get_theta() -> float:
	return 2.0 * asin(separation / (2.0 * radius))


func get_closest_position() -> Vector2:
	var children = _get_layout_children()
	if children.is_empty():
		return global_position

	var theta = get_theta()
	var idx = get_closest_idx()

	var angle = scroll_angle + (idx * theta)
	if flip:
		angle = PI - angle

	var center = circle_center + size * Vector2(0.0, 0.5)
	center = get_global_transform() * get_actual_center()
	return center + Vector2(cos(angle), sin(angle)) * radius


func scroll_to_index(idx: int):
	var children = _get_layout_children()
	idx = clamp(idx, 0, children.size() - 1)
	target_scroll_angle = -idx * get_theta()


func scroll_to_child(child: Control):
	var children = _get_layout_children()
	var idx := children.find(child)
	if idx != -1:
		scroll_to_index(idx)
		_on_scrolled()


func get_closest_idx() -> int:
	var theta = get_theta()
	var children = _get_layout_children()
	if children.is_empty():
		return -1

	var idx = round(-(scroll_angle) / theta)
	idx = clamp(idx, 0, children.size() - 1)
	return idx


func lerp_to_closest():
	var theta = get_theta()
	var children = _get_layout_children()
	if children.is_empty():
		return

	var idx = round(-scroll_angle / theta)
	idx = clampi(idx, 0, children.size() - 1)

	var snap = -idx * theta
	target_scroll_angle = lerpf(target_scroll_angle, snap, 0.03)


func _update_children():
	var children = _get_layout_children()
	var theta = get_theta()
	var center = get_actual_center()

	for i in range(children.size()):
		var child = children[i]
		var current_angle = scroll_angle + (i * theta)
		# Distance from selected idx# angular distance from center
		var angle_dist = abs(current_angle)
		if flip:
			current_angle = PI - current_angle
		var pos = center + Vector2(cos(current_angle), sin(current_angle)) * radius

		var dist = angle_dist / theta
		var _scale = pow(1.0 / (1.0 + dist * scale_multiplier), 1.5)
		child.pivot_offset_ratio = Vector2(0.0, 0.5) if not flip else Vector2(1.0, 0.5)
		var child_size = child.get_combined_minimum_size()
		fit_child_in_rect(child, Rect2(pos - (child_size / 2.0), child_size))
		child.scale = Vector2(_scale, _scale)


func _gui_input(event: InputEvent) -> void:
	var scroll_strength = 0.05
	if target_scroll_angle > 0 or target_scroll_angle < -(_get_layout_children().size() - 1) * get_theta():
		scroll_strength = 0.025
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT:
			if event.pressed:
				_dragging = true
				_last_mouse_pos = event.position
				_on_scrolled()
			else:
				_dragging = false
	elif event is InputEventMouseMotion and _dragging:
		var delta: Vector2 = event.relative
		_last_mouse_pos = event.position

		#TODO Generalize to x and y if exporting this
		target_scroll_angle += delta.y * drag_sensitivity
		_on_scrolled()

	if event.is_action_pressed("scroll_up"):
		target_scroll_angle += scroll_strength
		_on_scrolled()
	elif event.is_action_pressed("scroll_down"):
		target_scroll_angle -= scroll_strength
		_on_scrolled()


func _on_scrolled():
	_lerp_cooldown = max_lerp_cooldown


func get_actual_center() -> Vector2:
	var center = circle_center + (size * Vector2(0.0, 0.5))
	if flip:
		center.x = size.x - circle_center.x
	return center


func get_current_child() -> Node:
	return _get_layout_children()[get_closest_idx()]
