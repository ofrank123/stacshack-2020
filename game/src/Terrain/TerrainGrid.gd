extends Spatial

export var island_path: String
export var fog_path: String
export var cell_path: String

onready var nc: NetworkClient = get_node("/root")
onready var cell_temp := $Cell

const cell_size := 3.2
const padding := .8

var cells: Array
var size: int = 16

func setMap(cells_arr: Array, arr_size: int) -> void:
	cells = cells_arr
	size = arr_size

func updateMap() -> void:
	for x in range(0, size):
		for z in range(0, size):
			var cell := get_cell(x, z)


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
			if(randf() > .5):
				inst.add_terrain_base(island_path)
			else:
				inst.add_terrain_base(fog_path)
			cells[i] = inst

	
func _on_board_state(arr_server: Array) -> void:
	cells = arr_server

func _on_new_move(action: String, coords: Array) -> void:
	# Animate
	updateMap()
	pass

func _ready() -> void:
	# Signal connection
	nc.connect("board_state", self, "_on_board_state")
	nc.connect("new_move", self, "_on_new_move")

	init_cells()
