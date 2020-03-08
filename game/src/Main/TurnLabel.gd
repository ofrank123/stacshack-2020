extends Label

onready var main = get_tree().get_root().get_node("Main")

func _on_name_change(new_name):
	text = new_name

func _ready():
	main.connect("turn_change", self, "_on_name_change")
