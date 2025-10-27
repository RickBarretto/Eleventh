# Decks Service

This service manages the player's decks.

Overview
- Leader election: the node with the highest numeric `-id` is the leader.
- Leader handles mutating operations (POST /cards, DELETE /cards/{id}) and asynchronously replicates them to peers via POST /replicate.
- Followers forward mutating requests to the leader; GET /cards is served locally.

## API Endpoints

- `GET /cards`: 
    - List cards
- `POST /cards`: 
    - Add a card (JSON: `{"id":123,"name":"ace"}`); 
- `DELETE /cards/{id}`: 
    - Remove a card; 
- `POST /replicate`: 
    - Internal endpoint for replication
- `GET /status`: 
    - Node status and current leader

Some of those endpoints just returns values and others proxies the leader node. But for the user the behavior would be the same for any node.

## Real Usage

### Build
From the repository root (where `go.mod` is located), run:

```sh
go build -o decks.exe ./decks
```

### Starting

- **Node 1**
```powershell
./decks.exe -id=1 -addr=http://localhost:8001 -peers=1=http://localhost:8001,2=http://localhost:8002,3=http://localhost:8003
```

- **Node 2**
```powershell
./decks.exe -id=2 -addr=http://localhost:8002 -peers=1=http://localhost:8001,2=http://localhost:8002,3=http://localhost:8003
```

- **Node 3**
```powershell
./decks.exe -id=3 -addr=http://localhost:8003 -peers=1=http://localhost:8001,2=http://localhost:8002,3=http://localhost:8003
```

### CURL Usage

- Add a card

```powershell
curl -X POST http://localhost:8001/cards -H "Content-Type: application/json" -d '{"id":101,"name":"Ace"}'
```

- List cards from a follower

```powershell
curl http://localhost:8002/cards
```

- Delete a card

```powershell
curl -X DELETE http://localhost:8002/cards/101 -v
```

- List Cards

```powershell
curl http://localhost:8003/cards
```
