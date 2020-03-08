# WebSockets API

## Request Examples

```json
{
  "type": "Create",
  "username": "Hunter2"
}
```

```json
{
  "type": "Join",
  "game_id": 10021,
  "username": "Hunter2"
}
```

```json
{
  "type": "Action",
  "user_id": "db6cd159-82a1-4190-9eb0-ebd3df03ffb6",
  "kind": "Attack",
  "coordinate": [12, 41]
}
```

## Response Examples

```json
{
  "type": "Create",
  "user_id": "62933119-9ae7-4a74-9634-88310a6b9a28",
  "game_id": 48094
}
```

```json
{
  "type": "Join",
  "user_id": "08dab0d1-4383-47f7-8a4b-715bddf091fd"
}
```

```json
{
  "type": "Action",
  "last_action": {
    "user_id": "b6aaf29a-aceb-488d-87ac-04bec97d899a",
    "kind": "Attack",
    "coordinate": [12, 41]
  },
  "board": {
    "size": 16,
    "tiles": [
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      ...{
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      },
      {
        "kind": "Hidden",
        "owner": null,
        "defence": "None"
      }
    ]
  },
  "players": [
    {
      "id": "516c52f6-beb5-4d5c-9872-b8f9e4ecaec0",
      "name": "hunter2",
      "resources": 0,
      "color": 4290640121,
      "current": false
    },
    {
      "id": "9c5df371-c611-43b7-b3e1-62fa44d38838",
      "name": "player2",
      "resources": 0,
      "color": 4282727100,
      "current": false
    },
    {
      "id": "826f392b-40ed-452e-9776-0e521ac2d7ec",
      "name": "test",
      "resources": 0,
      "color": 4288998153,
      "current": false
    },
    {
      "id": "1d4d777c-c203-4501-8836-424e91c1b60d",
      "name": "john",
      "resources": 0,
      "color": 4278516739,
      "current": false
    }
  ],
  "expiry": "2020-03-08T08:25:55.066387Z"
}
```
