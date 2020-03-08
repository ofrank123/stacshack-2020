extends MarginContainer

var text: String = ""

onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")

func _on_text_changed(new_text: String):
	text = new_text

func _on_submit_button_down():
	_nc.create_game(text)
	get_tree().get_root().get_node("Main").add_child(load("res://src/Main/WaitScene.tscn").instance())
	queue_free()
