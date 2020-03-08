extends MarginContainer

func _on_create_button_down():
	get_tree().get_root().get_node("Main").add_child(load("res://src/Main/CreateMenu.tscn").instance())
	queue_free()


func _on_join_button_down():
	get_tree().get_root().get_node("Main").add_child(load("res://src/Main/JoinMenu.tscn").instance())
	queue_free()
