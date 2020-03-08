extends Label

onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")

func _on_connect(game_id, user_id):
	text = String(game_id)

func _ready():
	if _nc.game_id != null:
		text = String(_nc.game_id)
	_nc.connect("create", self, "_on_connect")
