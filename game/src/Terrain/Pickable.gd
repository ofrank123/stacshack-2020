extends Area

func _ready():
	self.connect("input_event", self, "_on_input_event")

func _on_input_event(camera, event, click_position, click_normal, shape_idx):
	if event is InputEventMouseMotion:
		get_node("..").get_surface_material(0).albedo_color = Color.white
