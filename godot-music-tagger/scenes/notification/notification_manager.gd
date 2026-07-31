extends Container
class_name NotificationManager
const NOTIF = preload("res://scenes/notification/notif.tscn")

@export var stack_spacing: float = 10
@export var move_lerp_speed: float = 20.0
var notifs: Array[PixelNotification] = []
var target_pos : Dictionary[PixelNotification, Vector2] = { }

#region Init
func _ready() -> void:
	Global.notif_manager = self

func create_notification(title:String, description:String, lifetime:int=5, progress_bar:bool=false) -> PixelNotification:
	var notif = NOTIF.instantiate() as PixelNotification
	notif.setup_notif(title, description, lifetime, progress_bar)
	notif.start_anim()
	notifs.push_front(notif)
	add_child(notif)
	notif.set_position(Vector2(-notif.get_combined_minimum_size().x, get_viewport_rect().size.y - notif.get_combined_minimum_size().y))
	notif.tree_exiting.connect(_on_notif_freed.bind(notif))
	return notif

func clear_notifications() -> void:
	for n in notifs:
		notifs.erase(n)
		n.disconnect("tree_exiting", _on_notif_freed)
		n.end_anim()

func _on_notif_freed(n:PixelNotification) -> void:
	notifs.erase(n)
#endregion
func _process(delta: float) -> void:
	if Input.is_action_just_pressed("5") and OS.is_debug_build():
		self.create_notification("Test", "THis is a test", 1, false)
		print("Making notif")
		
	for notif in notifs:
		if not target_pos.has(notif):
			continue
		var t := target_pos[notif]

		# smooth move in this Container's local space
		notif.position = notif.position.lerp(
			t,
			1.0 - exp(-move_lerp_speed * delta)
		)

func _notification(what):
	if what == NOTIFICATION_SORT_CHILDREN:
		_rebuild_targets()

func _remove_targets_for(n: PixelNotification) -> void:
	if target_pos.has(n):
		target_pos.erase(n)

func _rebuild_targets() -> void:
	# Recompute targets based on current ordering and sizes.
	# This is where you’d do "bottom-left stack" layout math.
	var viewport_size := get_viewport_rect().size

	# If you want consistent stacking from bottom upward:
	var y_cursor := stack_spacing

	for i in range(notifs.size()):
		var notif := notifs[i]
		if not is_instance_valid(notif):
			continue

		var notif_size := notif.get_combined_minimum_size()

		# Compute bottom-left stacked position:
		# - x is snapped to left (0)
		# - y grows upward from bottom by subtracting height
		var x := 0.
		var y := viewport_size.y - notif_size.y - y_cursor 

		target_pos[notif] = Vector2(x, y)

		y_cursor += notif_size.y + stack_spacing
		
