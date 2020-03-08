class_name NetworkClient
extends Node

export var websocket_url := "ws://fm208.host.cs.st-andrews.ac.uk:22220/"

# Move, Board State, Player State, Expiry
signal last_action(user_id, kind, coordinate)
signal board_state(board)
signal player_state(players)
signal expiry(datetime)

signal create(game_id, user_id)
signal join(user_id)

var _client := WebSocketClient.new()
var user_id: String
var game_id: int

func create_game(username: String) -> void:
	var packet = {
		"type": "Create",
		"username": username
		}
	_client.get_peer(1).put_packet(JSON.print(packet).to_ascii())

func join_game(g_id: int, username: String) -> void:
	var packet = {
		"type": "Join",
		"game_id": g_id,
		"username": username
		}
	_client.get_peer(1).put_packet(JSON.print(packet).to_ascii())

func make_action(kind: String, x: int, z: int) -> void:
	var packet = {
		"type": "Action",
		"user_id": user_id,
		"coordinate": [x, z]
		}
	_client.get_peer(1).put_packet(JSON.print(packet).to_ascii())

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
	pass

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
				emit_signal("join", user_id)
			"Create":
				print("Join Message Recevied:")
				user_id = p["user_id"]
				game_id = p["game_id"]
				print("User ID: ", user_id)
				print("Game ID: ", game_id)
				emit_signal("create", game_id, user_id)
			"Action":
				print("Action Received")
				emit_signal("board_state", p["board"])
				if(p["last_action"] != null):
					var action = p["last_action"]
					emit_signal("last_action", action["user_id"], action["kind"], action["coordinate"])
				else:
					emit_signal("last_action", "none", "", [])
				emit_signal("player_state", p["players"])
				emit_signal("expiry", p["expiry"])
	else:
		print("Error in data")


func _on_data_dummy() -> void:
	var file = File.new()
	file.open("res://src/Terrain/TerrainTest.json", File.READ)
	var content = file.get_as_text()
	file.close()

	var result = JSON.parse(content)
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
				emit_signal("board_state", p["board"])
				if(p["last_action"] != null):
					var action = p["last_action"]
					emit_signal("last_action", action["user_id"], action["kind"], action["coordinate"])
				else:
					emit_signal("last_action", "none", "", [])
				emit_signal("player_state", p["players"])
				emit_signal("expiry", p["expiry"])
	else:
		print("Error in data")


func _process(delta: float) -> void:
	_client.poll()
