extends Node

onready var _nc = $NetworkClient

var client_uuid: String
var players: Array

func get_player_uuid(uuid: String) -> Dictionary:
	for player in players:
		if player["id"].nocasecmp_to(uuid) == 0:
			return player

	return {}

func get_color(uuid: String) -> Color:
	if !get_player_uuid(uuid).empty():
		var rgba: int = get_player_uuid(uuid)["color"]
		return Color(rgba)
	return Color.white

func is_turn() -> bool:
	return get_player_uuid(client_uuid)["current"]

func _on_player_state(players_arr: Array) -> void:
	players = players_arr

func _ready() -> void:
	_nc.connect("player_state", self, "_on_player_state")

	# Twice to make sure everything is inited
	# Not necessary with regular connection
	# _nc._on_data_dummy()
	# _nc._on_data_dummy()
