extends Camera

export var speed = 25;
export var scroll_speed = 1;

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	handle_key_input(delta)
	pass

func _unhandled_input(event: InputEvent):
	if event is InputEventMouseButton:
		if event.button_index == BUTTON_WHEEL_UP:
			translation.y += scroll_speed
		if event.button_index == BUTTON_WHEEL_DOWN:
			translation.y += -scroll_speed

func handle_key_input(delta: float) -> void:
	if Input.is_action_pressed("camera_forward"):
		translation.x += speed * delta
		translation.z += speed * delta

	if Input.is_action_pressed("camera_backward"):
		translation.x += -speed * delta
		translation.z += -speed * delta

	if Input.is_action_pressed("camera_left"):
		translation.x += -speed * delta
		translation.z += speed * delta

	if Input.is_action_pressed("camera_right"):
		translation.x += speed * delta
		translation.z += -speed * delta
