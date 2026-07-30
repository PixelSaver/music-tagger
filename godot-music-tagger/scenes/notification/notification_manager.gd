extends Container
class_name NotificationManager
const NOTIF = preload("res://scenes/notification/notif.tscn")

var notifs: Array[PixelNotification] = []

#region Init
func _ready() -> void:
	Global.notif_manager = self

func create_notification(title:String, description:String, progress_bar:bool) -> PixelNotification:
	var notif = NOTIF.instantiate() as PixelNotification
	notif.setup_notif(title, description, progress_bar)
	notif.start_anim()
	return notif

func clear_notifications() -> void:
	for n in notifs:
		n.end_anim()
#endregion
