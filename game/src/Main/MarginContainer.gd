extends MarginContainer

onready var _main = get_tree().get_root().get_node("Main")
onready var _nc = _main.get_node("NetworkClient")

func _on_create_gui_input(event) -> void:
	if event is InputEventMouseButton:
		if event.pressed and event.button_index == BUTTON_LEFT:
			print("mouse press")
