extends Control
class_name PixelNotification

signal ended

@export_category("Nodes")
@export var title_label: RichTextLabel
@export var desc_label: RichTextLabel
@export var progress_bar: TextureProgressBar

var all_t: Array[Tweenable]
var t: Tween
var _elapsed := 0.0
var _lifetime := 5.0
var _has_progress_bar := false

#region Overrides
func _ready() -> void:
	all_t = PixelMenu.get_all_tweenables(self)

func _process(delta: float) -> void:
	if _lifetime > 0:
		_elapsed += delta
		if _elapsed >= _lifetime:
			ended.emit()
			queue_free()
#endregion

#region Notif Init
func setup_notif(title: String, description: String, has_progress_bar: bool) -> void:
	set_text(title, description)
	progress_bar.visible = has_progress_bar

func set_text(title: String, description: String) -> void:
	title_label.text = title
	desc_label.text = description

func set_progress(value: float, max_value: float) -> void:
	if not _has_progress_bar:
		return
	progress_bar.value = value
	progress_bar.max_value = max_value
#endregion

func start_anim() -> void:
	if t and t.is_running(): t.kill()
	t = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT).set_parallel(true)
	for table in all_t:
		t.tween_property(table, "tween_value", 1.0, 0.7)
	
func end_anim() -> void:
	if t and t.is_running(): t.kill()
	t = create_tween().set_ease(Tween.EASE_OUT).set_trans(Tween.TRANS_QUINT).set_parallel(true)
	for table in all_t:
		t.tween_property(table, "tween_value", 0.0, 0.7)
	await t.finished
	ended.emit()
	queue_free()
