# IntrastekApi

## Routes

### `Get /`

Just a ping pong route to verify that the api is up and running.

### `Get /asteks`

Get all the known asteks by the api.

### `Post /asteks`

Body:
```json
{
    "data":"Uuid"
}
```

Registers an astek to the api.

### `Get /asteks/<uuid>`

Get the informations on an astek if the api knows him

### `Post /asteks/<uuid>`

Body:
```json
{
    "data": {
        "interval": {
            "start": "timestamp",
            "end": "timestamp"
        },
        "type": "Indisponibility Type"
    }
}
```

Timestamps are following the `ISO 8601`
Indisponibility Type is one of :
- `"Private"`
- `{"activity":"FollowUp"}`
- `{"activity":"Bootstrap"}`
- `{"activity":"Review"}`
- `{"activity":"Keynote"}`
- `{"activity":"Surveillance"}`
- `{"activity":"Permanence"}`
