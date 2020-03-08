extends Area

onready var fogMaterial = load("res://src/Terrain/TerrainLib/Fog/FogMaterial.tres")
onready var hoverMaterial = load("res://src/Terrain/TerrainLib/Fog/HoverMaterial.tres")

var inHover: bool = false
var mouseMoved: bool = false

func _ready():
	self.connect("input_event", self, "_on_input_event")

func _on_input_event(camera, event, click_position, click_normal, shape_idx):
	if event is InputEventMouseMotion:
		inHover = true
		get_node("..").set_surface_material(0, hoverMaterial)

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		mouseMoved = true

func _process(delta: float):
	if(!inHover && mouseMoved):
		get_node("..").set_surface_material(0, fogMaterial)

	mouseMoved = false
	inHover = false
