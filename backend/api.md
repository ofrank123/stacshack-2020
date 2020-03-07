# WebSockets API

## Request

```json
{
    "game": "<UUIDV4>",
    "user": "<UUIDV4>",
    "kind": "Join" | "Fetch" | "Action": {
                                   "kind": "Explore" | "Improve" | "Attack"
                                   "coordinate": [X,Y]
                               }
}
```

### Examples

Request to attack `(12,3)`

```json
{
  "game": "38f7f65d-76bc-4d93-9226-ad8d1f342711",
  "user": "ae0ea178-3172-4b8c-9315-bd3e9942abfa",
  "kind": {
    "Action": {
      "kind": "Attack",
      "coordinate": [12, 3]
    }
  }
}
```

Request to join the game `9d0bddf3-c109-488d-81b7-449b6018bfbd` with user ID `cbfa7b5d-837a-4a2b-8dd8-0f44172721b3`.

```json
{
  "game": "9d0bddf3-c109-488d-81b7-449b6018bfbd",
  "user": "cbfa7b5d-837a-4a2b-8dd8-0f44172721b3",
  "kind": "Join"
}
```

Request to attack `(34,94)`

```json
{
  "game": "efeea5cc-38b1-4fd6-b6a9-1c5722fa3f3b",
  "user": "89eb0876-89e2-4bc6-b300-923668fa8cd3",
  "kind": {
    "Action": {
      "kind": "Explore",
      "coordinate": [34, 94]
    }
  }
}
```

## Response

Every response is the full game state

```json
{
  "board": {
    "size": 16,
    "tiles": [
      {
        "kind": "Empty",
        "owner": null
      },

        "kind": "Resource",
        "owner": "b017b439-d2ae-45b9-bd8b-91cd5f0b7c9f"
      },
      ...
      {
        "kind": "Empty",
        "owner": "23f77d23-8c9b-4cd2-9ad2-9e2d5fe71821"
      },
      {
        "kind": "Resource",
        "owner": null
      }
    ]
  },
  "players": [
    {
      "id": "b017b439-d2ae-45b9-bd8b-91cd5f0b7c9f",
      "resources": 1240,
      "color": 4289638500
    },
    {
      "id": "23f77d23-8c9b-4cd2-9ad2-9e2d5fe71821",
      "resources": 44,
      "color": 4288521666
    },
    {
      "id": "ea6e251f-cd71-4257-8b49-89dcac06be6b",
      "resources": 412,
      "color": 4280329463
    },
    {
      "id": "6aab400a-5b99-4155-8f8a-2c8e4b173512",
      "resources": 953,
      "color": 4281468536
    }
  ]
}
```
