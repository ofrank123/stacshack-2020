class_name NetworkClient
extends Node

export var websocket_url := "ws://fm208.host.cs.st-andrews.ac.uk:22220/"

# Move, Board State, Player State, Expiry
signal new_move(action, a)
signal board_state()
signal player_state
signal expiry

var _client := WebSocketClient.new()

func _ready() -> void:
	_client.connect("connection_closed", self, "_closed")
	_client.connect("connection_error", self, "_error")
	_client.connect("connection_established", self, "_connected")
	_client.connect("data_received", self, "_on_data")

	var err := _client.connect_to_url(websocket_url)
	if err != OK:
		print("Unable to connect")
		set_process(false)

func _closed(was_clean: bool = false) -> void:
	print("Closed, clean: ", was_clean)
	set_process(false)

func _connected(proto: String = "") -> void:
	var test_packet = {
		"type": "Move",
		"action": "Attack",
		"coordinate": [12, 42]
		}
	var json_string = JSON.print(test_packet)
	print(json_string)

	_client.get_peer(1).put_packet(JSON.print(test_packet).to_ascii())

func _on_data() -> void:
	print("Got data from server: ", _client.get_peer(1).get_packet().get_string_from_utf8())

func _process(delta: float) -> void:
	_client.poll()
