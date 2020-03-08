class_name TerrainCell

extends Spatial

onready var _main = get_tree().get_root().get_node("Main")
onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")

var base_path: NodePath
var top_path: NodePath
var light_path: NodePath

var kind: String = "Hidden"
var defence: String = "None"
var owner_id: String = ""

var updated_once = false

var x: int
var z: int

func clicked_on():
	print(owner_id)
	print(get_node("..").is_adjacent(x, z))
	if _main.is_turn() && get_node("..").is_adjacent(x, z):
		if kind == "Hidden":
			_nc.make_action("Explore", x, z)
		if !_main.is_player(owner_id):
			_nc.make_action("Attack", x, z)
		if _main.is_player(owner_id) and defence != "High":
			_nc.make_action("Improve", x, z)

func requires_update(server_cell: Dictionary):
	if !updated_once:
		updated_once = true
		return updated_once
	else:
		return kind != server_cell["kind"] or defence != server_cell["defence"] or owner_id != server_cell["owner"]

func add_terrain_base(instance_path: String) -> void:
	remove_terrain_base()
	var instance = load(instance_path).instance()
	# Make material unique
	instance.set_surface_material(0, instance.get_surface_material(0).duplicate())
	# Rotate a random amount
	instance.rotation.y = (PI / 2) * (randi() % 4)
	add_child(instance)
	base_path = instance.get_path()

func remove_terrain_base() -> void:
	if(base_path != ""):
		get_node(base_path).queue_free()

func add_terrain_top(instance_path: String, color: Color = Color.white) -> void:
	remove_terrain_top()
	var instance = load(instance_path).instance()
	# Make material unique
	instance.set_surface_material(0, instance.get_surface_material(0).duplicate())
	print(color)
	instance.get_surface_material(0).albedo_color = color
	# Rotate a random amount
	instance.rotation.y = (PI / 2) * (randi() % 4)
	add_child(instance)
	top_path = instance.get_path()

func remove_terrain_top() -> void:
	if(top_path != ""):
		get_node(top_path).queue_free()

func add_light(instance_path: String, color: Color = Color.white) -> void:
	remove_light()
	var instance = load(instance_path).instance()

	instance.light_color = color
	add_child(instance)
	light_path = instance.get_path()


func remove_light() -> void:
	if(light_path != ""):
		get_node(light_path).queue_free()
