extends MarginContainer


var usertext: String = ""
var gidtext: String = ""

onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")

func _on_GameID_text_changed(new_text: String):
	gidtext = new_text

func _on_User_text_changed(new_text: String):
	usertext = new_text

func _on_submit_button_down():
	_nc.join_game(gidtext.to_int(), usertext)
	get_tree().get_root().get_node("Main").add_child(load("res://src/Main/JoinWaitScene.tscn").instance())
	queue_free()
