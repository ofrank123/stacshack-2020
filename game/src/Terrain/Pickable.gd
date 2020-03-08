extends Area

onready var hoverMaterial = load("res://src/Terrain/HoverMaterial.tres")

var inHover: bool = false
var mouseMoved: bool = false
var normalMaterial: Material


func _ready():
	self.connect("input_event", self, "_on_input_event")
	normalMaterial = get_node("..").get_surface_material(0)

func _on_input_event(camera, event, click_position, click_normal, shape_idx):
	if event is InputEventMouseMotion:
		inHover = true
		get_node("..").set_surface_material(0, hoverMaterial)
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_LEFT and event.pressed:
			get_node("../..").clicked_on()

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		mouseMoved = true
	if event is InputEventKey:
		mouseMoved = true

func _process(delta: float):
	if(!inHover && mouseMoved && normalMaterial != null):
		get_node("..").set_surface_material(0, normalMaterial)

	mouseMoved = false
	inHover = false
