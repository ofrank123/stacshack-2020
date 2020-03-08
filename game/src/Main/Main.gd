extends Node

onready var _nc = $NetworkClient

signal resource_change(new_res)
signal turn_change(new_name)

var client_uuid: String
var players: Array
var resources: int = 0

func get_player_uuid(uuid: String) -> Dictionary:
	for player in players:
		if player["id"].nocasecmp_to(uuid) == 0:
			return player

	return {}

func get_current_player_name() -> String:
	for player in players:
		if player["current"] == true:
			return player["name"]
	return "Nobody"

func get_color(uuid: String) -> Color:
	if !get_player_uuid(uuid).empty():
		var rgba: int = get_player_uuid(uuid)["color"]
		return Color(rgba)
	return Color.white

func is_turn() -> bool:
	return get_player_uuid(client_uuid)["current"]

func is_player(uuid: String) -> bool:
	return uuid == client_uuid

func _on_player_state(players_arr: Array) -> void:
	players = players_arr
	resources = get_player_uuid(client_uuid)["resources"]
	resources = 10
	emit_signal("resource_change", resources)
	emit_signal("turn_change", get_current_player_name())

func _on_create(_game_id, user_id):
	client_uuid = user_id

func _on_join(user_id):
	client_uuid = user_id

func _ready() -> void:
	_nc.connect("player_state", self, "_on_player_state")
	_nc.connect("create", self, "_on_create")
	_nc.connect("create", self, "_on_join")
	# Twice to make sure everything is inited
	# Not necessary with regular connection
	# _nc._on_data_dummy()
	# _nc._on_data_sawdummy()
