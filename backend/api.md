# WebSockets API

## Request

### Examples

```json
{
  "type": "Create",
  "username": "Hunter2"
}
```

```json
{
  "type": "Join",
  "username": "Hunter2",
  "game_id": "929a3d9d-ca90-44d9-b583-ae0dd5ad9fbd"
}
```

```json
{
  "type": "Move",
  "action": "Attack",
  "coordinate": [12, 41]
}
```

## Response

### Examples

```json
{
  "type": "Create",
  "user_id": "13a94ae5-aa42-4fe5-8bc9-fef5050cbc8a",
  "game_id": "e3ca0fbc-9113-4027-acd0-971ec0838a00"
}
```

```json
{
  "type": "Join",
  "user_id": "5141d1f2-e39a-4dbf-a3e3-52c2228a93fb"
}
```

```json
{
  "type": "Move",
  "last_move": {
    "action": "Attack",
    "coordinate": [12, 41]
  },
  "board": {
    "size": 16,
    "tiles": [
      {
        "kind": "Empty",
        "owner": null,
        "discovered": false
      },
      {
        "kind": "Empty",
        "owner": null,
        "discovered": false
      },
      ...{
        "kind": "Empty",
        "owner": null,
        "discovered": false
      },
      {
        "kind": "Empty",
        "owner": null,
        "discovered": false
      }
    ]
  },
  "players": [
    {
      "id": "fa6cbe41-5a3b-4c7b-b9bc-abd4d56e2037",
      "resources": 0,
      "color": 4287475721,
      "current": false
    }
  ],
  "expiry": "2020-03-07T18:59:39.420393Z"
}
```
