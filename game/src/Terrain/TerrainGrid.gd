extends Spatial

export var island_path: String
export var fog_path: String
export var ore_path: String
export var cell_path: String

export var defence_low_path: String
export var defence_high_path: String

export var light_path: String

onready var _nc = get_tree().get_root().get_node("Main/NetworkClient")
onready var _main = get_tree().get_root().get_node("Main")

const cell_size := 3.2
const padding := .8

var cells: Array
var server_board: Array
var size: int = 16

func setMap(cells_arr: Array, arr_size: int) -> void:
	cells = cells_arr
	size = arr_size

func updateMap() -> void:
	for x in range(0, size):
		for z in range(0, size):
			var server_cell := get_server_cell(x, z)
			var cell := get_cell(x, z)
			var owner = server_cell["owner"]
			var color := Color.white
			if owner != null:
				color =_main.get_color(owner)

			if !server_cell.empty() && cell.requires_update(server_cell):
				print("Test")
				cell.kind = server_cell["kind"]
				cell.defence = server_cell["defence"]
				if owner != null:
					cell.owner_id = owner
					cell.add_light(light_path, color)
				match server_cell["kind"]:
					"Hidden":
						cell.add_terrain_base(fog_path)
					"Normal":
						cell.add_terrain_base(island_path)
					"Resource":
						cell.add_terrain_base(ore_path)
					_:
						cell.add_terrain_base(fog_path)
				match server_cell["defence"]:
					"None":
						cell.remove_terrain_top()
					"Low":
						cell.add_terrain_top(defence_low_path, color)
					"High":
						cell.add_terrain_top(defence_high_path, color)


func get_server_cell(x: int, z: int) -> Dictionary:
	if server_board != null:
		return server_board[(x * size) + z]
	return {}

func get_cell(x: int, z: int) -> TerrainCell:
	if (x * size) + z < (size * size):
		return cells[(x * size) + z]
	else:
		return null

func init_cells() -> void:
	cells = []
	cells.resize(size * size)
	for x in range(0, size):
		for z in range(0, size):
			var i := (x * size) + z
			var inst = load(cell_path).instance()
			add_child(inst)
			inst.translate(Vector3(x * (cell_size + padding), 0, z * (cell_size + padding)))
			inst.add_terrain_base(island_path)
			cells[i] = inst

	
func _on_board_state(arr_server: Dictionary) -> void:

	if arr_server["size"] != size:
		size = arr_server["size"]
		init_cells()
	server_board = arr_server["tiles"]

func _on_new_move(action: String, kind: String, coords: Array) -> void:
	# Animate
	updateMap()
	pass

func _ready() -> void:

	init_cells()

	_nc.connect("board_state", self, "_on_board_state")
	_nc.connect("last_action", self, "_on_new_move")

