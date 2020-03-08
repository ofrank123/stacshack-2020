class_name TerrainCell

extends Spatial

var base_path: NodePath
var top_path: NodePath

func add_terrain_base(instance_path: String) -> void:
	if(base_path != ""):
		remove_terrain_base()
	var instance = load(instance_path).instance()
	# Make material unique
	instance.set_surface_material(0, instance.get_surface_material(0).duplicate())
	# Rotate a random amount
	instance.rotation.y = (PI / 2) * (randi() % 4)
	add_child(instance)
	base_path = instance.get_path()

func remove_terrain_base() -> void:
	get_node(base_path).queue_free()

func add_terrain_top(instance_path: String) -> void:
	pass

func remove_terrain_top() -> void:
	pass
