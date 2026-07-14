extends Label

func _process(_delta: float) -> void:
	var mem = Performance.get_monitor(Performance.MEMORY_STATIC)
	var queue = Performance.get_monitor(Performance.MEMORY_MESSAGE_BUFFER_MAX)

	text = "RAM: %.2f MB\nQueue: %.2f MB" % [
		mem / 1024.0 / 1024.0,
		queue / 1024.0 / 1024.0
	]
