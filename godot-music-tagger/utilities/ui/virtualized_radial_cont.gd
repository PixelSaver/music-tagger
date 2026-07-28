@tool
extends Container

class_name VirtualizedRadialContainer

signal selected_item_changed(idx: int)
signal bind_item(control:Control, idx:int)


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
@export var item_count: int = 0:
	set(val):
		item_count = max(val, 0)
		_recalculate_pool_size()
		queue_sort()
@export var visibility_window := 15:
	set(val):
		visibility_window = max(val, 0)
		_recalculate_pool_size()
		queue_sort()
@export var target_scroll_angle := 0.0:
	set(val):
		target_scroll_angle = val
		queue_sort()
		if Engine.is_editor_hint():
			_update_children()
@export var scroll_active := true :
	set(val):
		scroll_active = val
		if Engine.is_editor_hint():
			_update_children()
@export var drag_sensitivity := 0.005
@export_range(0.0, 1.0) var scroll_accel := 1.0
## Proportion taken off the scale of neighboring children.
## Children farther away from current angle are this much smaller
@export var scale_multiplier := 0.1
@export var scroll_bar: VScrollBar
@export_category("Container Exclusion")
@export var excluded: Array[Node] = []
@export var max_lerp_cooldown := 0.6

var scroll_angle := 0.0
var _pool: Array[Control] = []
var _pool_start_idx := 0
var _current_selected_idx := -1
var _last_scrolled_angle : float = INF
#var _previous_start: int = INT32_MAX
#var _previous_end: int = INT32_MIN
var _lerp_cooldown: float
var _bound_item_idx: Array[int] = []

# Dragging
var _dragging := false
var _last_mouse_pos := Vector2.ZERO

#region Public stuff
func get_children_count() -> int:
	return item_count

func add_pool_control(control:Control) -> void:
	control.visible = false
	_pool.append(control)
	add_child(control)

func clear_pool() -> void:
	for child in _pool:
		child.queue_free()
	_pool.clear()


func scroll_to_index(idx: int):
	if item_count <= 0:
		return
	idx = clampi(idx, 0, item_count - 1)
	target_scroll_angle = -idx * get_theta()

func get_closest_idx() -> int:
	if item_count <= 0:
		return -1

	var idx = round(-(scroll_angle) / get_theta())
	return clampi(idx, 0, item_count - 1)

#endregion

#region initialization
func _enter_tree() -> void:
	if !scroll_bar: 
		scroll_bar = VScrollBar.new()
		add_child(scroll_bar)
		excluded.append(scroll_bar)
	scroll_bar.z_index = 100


func _ready() -> void:
	self.scroll_angle = 0
	self.target_scroll_angle = self.scroll_angle
	_lerp_cooldown = max_lerp_cooldown
	scroll_bar.scrolling.connect(func():
		self.scroll_to_index(int(scroll_bar.value))
	)
	_recalculate_pool_size()
	#self.child_entered_tree.connect(func(node:Node):
		#if node is not Control: return
		#if excluded.has(node): return
		#if not _pool.has(node):
			#_pool.append(node)
			#_recalculate_pool_size()
	#)
	#self.child_exiting_tree.connect(func(node:Node):
		#if _pool.has(node):
			#_pool.erase(node)
	#)
	_bound_item_idx.resize(_pool.size())
	for i in _bound_item_idx.size():
		_bound_item_idx[i] = -1
#endregion

#region Pool stuff
func _get_required_pool_size() -> int:
	if item_count <= 0:
		return 0
	return min(
		item_count,
		visibility_window * 2 + 1
	)


func _recalculate_pool_size() -> void:
	var req := _get_required_pool_size()
	
	if req == _pool.size():
		return
	
	if req < _pool.size():
		while _pool.size() > req:
			var child = _pool.pop_back() as Control
			if !is_instance_valid(child): continue
			if child: child.queue_free()
	else:
		push_warning("Radial container requires %s controls but only has %s" % [req, _pool.size()])
	
	#_update_children()

func _bind_pool_item(pool_idx:int, item_idx:int) -> void:
	var control := _pool[pool_idx]
	
	if item_idx < 0 or item_idx >= item_count:
		control.hide()
		return
	
	if _bound_item_idx[pool_idx] == item_idx:
		control.show()
		return
	
	if item_idx < 0 or item_idx >= item_count:
		control.hide()
		return
	
	_bound_item_idx[pool_idx] = item_idx
	control.show()
	_bind_emit(control, item_idx)

func _bind_emit(control:Control, item_idx:int): 
	bind_item.emit(control, item_idx)

#endregion


func _process(delta: float) -> void:
	if item_count <= 0: return
	
	var min_limit = -(item_count - 1) * get_theta()
	var max_limit = 0.0

	var is_overshooting = target_scroll_angle > max_limit or target_scroll_angle < min_limit

	if is_overshooting:
		var target = clampf(target_scroll_angle, min_limit, max_limit)
		target_scroll_angle = lerpf(target_scroll_angle, target, delta * 10.0)
		
	scroll_angle = lerpf(scroll_angle, target_scroll_angle, delta * 5.0)
	
	if !(abs(scroll_angle - _last_scrolled_angle) < .000001) or !(abs(target_scroll_angle - scroll_angle) < .00001):
		_update_children()
		_update_scrollbar()
		
	var idx := get_closest_idx()
	if idx != _current_selected_idx:
		_current_selected_idx = idx
		selected_item_changed.emit(idx)
		
	if Engine.is_editor_hint(): return
	
	_lerp_cooldown -= delta
	if _lerp_cooldown < 0.0:
		lerp_to_closest()
	_last_scrolled_angle = scroll_angle


func _update_scrollbar():
	if scroll_bar == null or Engine.is_editor_hint():
		return
	
	scroll_bar.visible = item_count > visibility_window
	if !scroll_bar.visible:
		return
	var vw = visibility_window * 2 + 1
	scroll_bar.min_value = 0
	scroll_bar.max_value = max(0, item_count - 1 + vw) 
	scroll_bar.page = vw
	scroll_bar.set_value_no_signal(get_closest_idx())

func _update_children():
	if _pool.is_empty(): return
	if scroll_bar:
		var width := 12.0
		scroll_bar.position = Vector2(size.x - width, 0)
		scroll_bar.size = Vector2(width, size.y)
	
	var theta = get_theta()
	var center = get_actual_center()
	var closest_idx = get_closest_idx()
	if closest_idx < 0: return
	
	var start = maxi(closest_idx - visibility_window, 0)
	var end = mini(closest_idx + visibility_window + 1, item_count)
	_pool_start_idx = start
	
	for i in range(_pool.size()):
		var true_idx: int = start + i
		if true_idx >= end:
			_pool[i].hide()
			continue
		_bind_pool_item(i, true_idx)
		var child = _pool[i]
		
		var current_angle = scroll_angle + (true_idx * theta)
		var angle_dist = abs(current_angle)
		if flip:
			current_angle = PI - current_angle
		var pos = center + Vector2(cos(current_angle), sin(current_angle)) * radius

		var dist = angle_dist / theta
		var _scale = pow(1.0 / (1.0 + dist * scale_multiplier), 1.5)
		#child.pivot_offset_ratio = Vector2(0.0, 0.5) if not flip else Vector2(1.0, 0.5)
		child.pivot_offset_ratio = Vector2(0.0, 0.5) if flip else Vector2(1.0, 0.5)
		var child_size = child.get_combined_minimum_size()
		fit_child_in_rect(child, Rect2(pos - (child_size / 2.0), child_size))
		child.scale = Vector2(_scale, _scale)



func _notification(what):
	if what == NOTIFICATION_SORT_CHILDREN:
		_update_children()



## Angle separation between two children
func get_theta() -> float:
	return 2.0 * asin(separation / (2.0 * radius))


func get_closest_position() -> Vector2:
	if _pool.is_empty():
		return global_position

	var theta = get_theta()
	var idx = get_closest_idx()

	var angle = scroll_angle + (idx * theta)
	if flip:
		angle = PI - angle

	var center = circle_center + size * Vector2(0.0, 0.5)
	center = get_global_transform() * get_actual_center()
	return center + Vector2(cos(angle), sin(angle)) * radius



#func scroll_to_child(child: Control):
	#var children = _get_layout_children()
	#var idx := children.find(child)
	#if idx != -1:
		#scroll_to_index(idx)
		#_on_scrolled()


func lerp_to_closest():
	if item_count <= 0: return
	var theta = get_theta()
	
	var idx = round(-scroll_angle / theta)
	idx = clampi(idx, 0, item_count - 1)
	
	var snap = -idx * theta
	target_scroll_angle = lerpf(target_scroll_angle, snap, 0.03)


func _gui_input(event: InputEvent) -> void:
	if item_count <= 0: return
	var scroll_strength = 0.05
	var boost = clampf(exp(1.5*scroll_accel*abs(target_scroll_angle - scroll_angle)), 1, 5)
	if target_scroll_angle > 0 or target_scroll_angle < -(item_count - 1) * get_theta():
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
		target_scroll_angle += delta.y * drag_sensitivity * boost
		_on_scrolled()

	if event.is_action_pressed("scroll_up"):
		target_scroll_angle += scroll_strength * boost
		_on_scrolled()
	elif event.is_action_pressed("scroll_down"):
		target_scroll_angle -= scroll_strength * boost
		_on_scrolled()


func _on_scrolled():
	_lerp_cooldown = max_lerp_cooldown


func get_actual_center() -> Vector2:
	var center = circle_center + (size * Vector2(0.0, 0.5))
	if flip:
		center.x = size.x - circle_center.x
	return center


func get_current_control() -> Control:
	var idx := get_closest_idx()
	if idx < 0: return null
	var pool_idx = idx - _pool_start_idx
	if pool_idx < 0 or pool_idx >= _pool.size(): return null
	return _pool[pool_idx]
	
