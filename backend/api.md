# WebSockets API

## Request

### Examples

```json
{
  "Create": {
    "username": "Hunter2"
  }
}
```

```json
{
  "Join": {
    "username": "Hunter2",
    "game_id": "96707aea-0e52-4659-ae33-7e4b0bf8e94c"
  }
}
```

```json
{
  "Move": {
    "action": "Attack",
    "coordinate": [12, 41]
  }
}
```

## Response

### Examples

```json
{
  "Create": {
    "user_id": "620c80ed-af24-43ca-b20d-681c5352bcc6",
    "game_id": "2cf50c58-4ff1-42c9-af68-2deb91fe027e"
  }
}
```

```json
{
  "Join": {
    "user_id": "7b775b01-9210-495a-82f3-7e13a0f89838"
  }
}
```

```json
{
  "Move": {
    "last_move": {
      "action": "Attack",
      "coordinate": [12, 41]
    },
    "board": {
      "size": 16,
      "tiles": [
        {
          "kind": "Resource",
          "owner": null,
          "discovered": false
        },
        {
          "kind": "Empty",
          "owner": "7b775b01-9210-495a-82f3-7e13a0f89838",
          "discovered": true
        },
        ...{
          "kind": "Empty",
          "owner": null,
          "discovered": true
        },
        {
          "kind": "Resource",
          "owner": "7b775b01-9210-495a-82f3-7e13a0f89838",
          "discovered": false
        }
      ]
    },
    "players": [
      {
        "id": "56b8e99a-c525-456c-8811-70d5f867fb97",
        "resources": 5325,
        "color": 4293689558,
        "current": false
      },
      {
        "id": "0baa88ef-5acc-4d68-9755-7cd9dcfde8d2",
        "resources": 0,
        "color": 4286984594,
        "current": false
      }
    ],
    "expiry": "2020-03-07T17:09:32.189539Z"
  }
}
```
