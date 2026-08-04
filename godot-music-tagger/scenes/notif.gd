extends MarginContainer
class_name PixelNotification

signal ended

@export_category("Nodes")
@export var title_label: RichTextLabel
@export var desc_label: RichTextLabel
@export var progress_bar: TextureProgressBar

@export var all_t: Array[Tweenable]
var t: Tween
var _elapsed := 0.0
var _lifetime := 5.0
var _has_progress_bar := false
var _ended := false
var _desc := ""

#region Overrides
func _ready() -> void:
	#all_t = PixelMenu.get_all_tweenables(self)
	for _t in all_t:
		_t.tween_value = 1.0

func _process(delta: float) -> void:
	if _lifetime > 0:
		_elapsed += delta
		if _elapsed >= _lifetime:
			end_anim()

func _gui_input(event: InputEvent) -> void:
	if event.is_action_pressed("l_click"):
		self.end_anim()
#endregion

#region Public api
func end_notif() -> void:
	self.end_anim()
#endregion

#region Notif Init
func setup_notif(title: String, description: String, lifetime:int, has_progress_bar: bool) -> void:
	_lifetime = lifetime
	set_text(title, description)
	progress_bar.visible = has_progress_bar
	_has_progress_bar = has_progress_bar

func set_text(title: String, description: String) -> void:
	title_label.text = title
	desc_label.text = description
	_desc = description

func set_progress(value: float, max_value: float) -> void:
	if not _has_progress_bar:
		return
	desc_label.text = "%s | progress: %.1f%%" % [_desc, value/max_value*100]
	progress_bar.value = value
	progress_bar.max_value = max_value

func set_remaining_lifetime(remaining:float) -> void:
	_lifetime = remaining
#endregion

func start_anim() -> void:
	if all_t.size() == 0: return
	if t and t.is_running(): t.kill()
	t = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT).set_parallel(true)
	for table in all_t:
		t.tween_property(table, "tween_value", 0.0, 0.7)
	
func end_anim() -> void:
	if _ended: return
	_ended = true
	if all_t.size() > 0: 
		if t and t.is_running(): t.kill()
		t = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT).set_parallel(true)
		for table in all_t:
			t.tween_property(table, "tween_value", 1.0, 0.7)
		print("Exiting tween edone")
		await t.finished
	ended.emit()
	queue_free()
