extends Label

onready var main = get_tree().get_root().get_node("Main")

func _on_resource_change(new_res: int):
	text = String(new_res)

func _ready():
	main.connect("resource_change", self, "_on_resource_change")
