class_name NetworkClient
extends Node

export var websocket_url := "ws://fm208.host.cs.st-andrews.ac.uk:22220/"

# Move, Board State, Player State, Expiry
signal last_action(user_id, kind, coordinate)
signal board_state(board)
signal player_state(players)
signal expiry(datetime)

var _client := WebSocketClient.new()
var user_id: String
var game_id: int

func _ready() -> void:
	_client.connect("connection_closed", self, "_closed")
	_client.connect("connection_error", self, "_error")
	_client.connect("connection_established", self, "_connected")
	_client.connect("data_received", self, "_on_data")

	var err := _client.connect_to_url(websocket_url)
	if err != OK:
		print("Unable to connect")
		set_process(false)

func _error():
	print("Error")

func _closed(was_clean: bool = false) -> void:
	print("Closed, clean: ", was_clean)
	set_process(false)

func _connected(proto: String = "") -> void:
	var test_packet = {
		"type": "Create",
		"username": "ofrank"
		}
	var json_string = JSON.print(test_packet)
	print(json_string)

	_client.get_peer(1).put_packet(JSON.print(test_packet).to_ascii())

func _on_data() -> void:
	# print("Got data from server: ", _client.get_peer(1).get_packet().get_string_from_utf8())
	var result = JSON.parse(_client.get_peer(1).get_packet().get_string_from_utf8())
	var p = result.result
	if typeof(p) == TYPE_DICTIONARY:
		match p["type"]:
			"Join":
				print("Join Message Recevied:")
				user_id = p["user_id"]
				print("User ID: ", user_id)
			"Create":
				print("Join Message Recevied:")
				user_id = p["user_id"]
				game_id = p["game_id"]
				print("User ID: ", user_id)
				print("Game ID: ", game_id)
			"Action":
				print("Action Received")
				emit_signal("last_action", p["user_id"], p["kind"], p["coordinate"])
				emit_signal("board_state", p["board"])
				emit_signal("player_state", p["players"])
				emit_signal("expiry", p["expiry"])


	else:
		print("Error in data")

func _process(delta: float) -> void:
	_client.poll()
