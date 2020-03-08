extends MarginContainer

onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")

func _on_game_start(user_id, kind, coordinate):
	get_tree().get_root().get_node("Main/Spatial").visible = true
	get_tree().get_root().get_node("Main/GUI").visible = true
	queue_free()

func _ready() -> void:
	_nc.connect("last_action", self, "_on_game_start")
