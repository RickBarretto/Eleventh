package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Simple Card Model that simulates one
// 
// This model accomplishes its purpose for this proof-of-concept,
// but on real model this should be more complete and also with
// some attributes important for the game.
type Card struct {
    ID   int    `json:"id"`
    Name string `json:"name"`
}

// In-Memoty Deck
//
// This deck could use a distributed database,
// on real implementations.
type Deck struct {
    mu    sync.RWMutex
    cards map[int]Card
}

func NewDeck() *Deck {
    return &Deck {
        cards: make(map[int]Card),
    }
}

func (deck *Deck) Add(card Card) {
    deck.mu.Lock()
    defer deck.mu.Unlock()

    deck.cards[card.ID] = card
}

func (deck *Deck) Remove(card_id int) {
    deck.mu.Lock()
    defer deck.mu.Unlock()

    delete(deck.cards, card_id)
}

func (deck *Deck) List() []Card {
    deck.mu.RLock()
    defer deck.mu.RUnlock()

    result := make([]Card, 0, len(deck.cards))

    for _, card := range deck.cards {
        result = append(result, card)
    }
    return result
}

// ReplicateRequest used by leader to replicate an operation
type ReplicateRequest struct {
    Op   string `json:"op"` // "add" or "remove"
    Card Card   `json:"card"`
}

// Node holds node info and state
type Node struct {
    id         int
    addr       string
    peers      map[int]string // id -> addr
    leaderID   int
    leaderAddr string
    deck       *Deck
    client     *http.Client
    mu         sync.RWMutex // protects leaderID/leaderAddr
}

func NewNode(id int, addr string, peers map[int]string) *Node {
    node := &Node {
        id:    id,
        addr:  addr,
        peers: peers,
        deck:  NewDeck(),
        client: &http.Client{
            Timeout: 5 * time.Second,
        },
    }

    node.electLeader()
    return node
}

func (node *Node) isLeader() bool {
    node.mu.RLock()
    defer node.mu.RUnlock()
    
    return node.leaderID == node.id
}

/// Elect a leader via bully algorithm.
///
/// The highest available ID is the leader.
func (node *Node) electLeader() {
    highestID := node.id
    highestAddress := node.addr
    
    for peer_id, peer_address := range node.peers {
        if peer_id > highestID {
            highestID = peer_id
            highestAddress = peer_address
        }
    }
    node.mu.Lock()
    node.leaderID = highestID
    node.leaderAddr = highestAddress
    node.mu.Unlock()
}

func (node *Node) startLeaderLoop() {
    ticker := time.NewTicker(3 * time.Second)
    go func() {
        for range ticker.C {
            node.electLeader()
        }
    }()
}

/// Send commands to other peers to replace the same behavior.
func (node *Node) replicateToFollowers(request ReplicateRequest) {
    data, _ := json.Marshal(request)

    for peerID, peerAddress := range node.peers {
        if peerID == node.id {
            continue
        }

        go func(address string, id int) {
            url := strings.TrimRight(address, "/") + "/replicate"
            httpRequest, err := http.NewRequest("POST", url, bytes.NewReader(data))
            
            if err != nil {
                log.Printf("replicate: create request to %s: %v", address, err)
                return
            }
            
            httpRequest.Header.Set("Content-Type", "application/json")
            
            response, err := node.client.Do(httpRequest)
            if err != nil {
                log.Printf("replicate: POST %s failed: %v", url, err)
                return
            }
            
            io.Copy(io.Discard, response.Body)
            response.Body.Close()
            
            if response.StatusCode >= 300 {
                log.Printf("replicate: non-2xx from %s: %s", url, response.Status)
            }
        }(peerAddress, peerID)
    }
}

/// Forward incoming requests to the leader and proxy the response
func (node *Node) forwardToLeader(
    writer http.ResponseWriter, 
    request *http.Request,
) {

    node.mu.RLock()
    leader := node.leaderAddr
    node.mu.RUnlock()

    if leader == "" {
        http.Error(writer, "no leader known", http.StatusServiceUnavailable)
        return
    }

    // build URL to leader
    destinationURL := strings.TrimRight(leader, "/") + request.URL.Path

    // read body
    var bodyBytes []byte
    if request.Body != nil {
        data, _ := io.ReadAll(request.Body)
        bodyBytes = data
    }

    req, err := http.NewRequest(request.Method, destinationURL, bytes.NewReader(bodyBytes))
    if err != nil {
        http.Error(writer, "failed to create request to leader", http.StatusInternalServerError)
        return
    }

    req.Header = request.Header.Clone()
    resp, err := node.client.Do(req)
    if err != nil {
        http.Error(writer, "leader unreachable: "+err.Error(), http.StatusServiceUnavailable)
        return
    }
    defer resp.Body.Close()

    for k, v := range resp.Header {
        writer.Header()[k] = v
    }

    writer.WriteHeader(resp.StatusCode)
    io.Copy(writer, resp.Body)
}

func (node *Node) handleGetCards(
    writer http.ResponseWriter, 
    request *http.Request,
) {
    cards := node.deck.List()
    writer.Header().Set("Content-Type", "application/json")
    json.NewEncoder(writer).Encode(cards)
}

func (node *Node) handlePostCard(
    writer http.ResponseWriter, 
    request *http.Request,
) {
    if !node.isLeader() {
        node.forwardToLeader(writer, request)
        return
    }

    var c Card
    if err := json.NewDecoder(request.Body).Decode(&c); err != nil {
        http.Error(writer, "invalid json", http.StatusBadRequest)
        return
    }

    node.deck.Add(c)

    // replicate
    node.replicateToFollowers(ReplicateRequest{Op: "add", Card: c})
    writer.WriteHeader(http.StatusCreated)
    json.NewEncoder(writer).Encode(c)
}

func (node *Node) handleDeleteCard(
    writer http.ResponseWriter, 
    request *http.Request,
) {
    // path expected: /cards/{id}
    parts := strings.Split(strings.Trim(request.URL.Path, "/"), "/")

    if len(parts) != 2 {
        http.Error(writer, "bad path", http.StatusBadRequest)
        return
    }

    id, err := strconv.Atoi(parts[1])

    if err != nil {
        http.Error(writer, "invalid id", http.StatusBadRequest)
        return
    }

    if !node.isLeader() {
        node.forwardToLeader(writer, request)
        return
    }

    node.deck.Remove(id)
    node.replicateToFollowers(ReplicateRequest{Op: "remove", Card: Card{ID: id}})
    writer.WriteHeader(http.StatusNoContent)
}

func (node *Node) handleReplicate(
    writer http.ResponseWriter, 
    request *http.Request,
) {

    var req ReplicateRequest
    if err := json.NewDecoder(request.Body).Decode(&req); err != nil {
        http.Error(writer, "invalid replicate payload", http.StatusBadRequest)
        return
    }

    switch req.Op {
        case "add":
            node.deck.Add(req.Card)
        case "remove":
            node.deck.Remove(req.Card.ID)
        default:
            http.Error(writer, "unknown op", http.StatusBadRequest)
            return
    }

    writer.WriteHeader(http.StatusOK)
}

func (node *Node) handleStatus(
    writer http.ResponseWriter, 
    request *http.Request,
) {

    node.mu.RLock()

    leaderID := node.leaderID
    leaderAddr := node.leaderAddr
    node.mu.RUnlock()
    out := map[string]interface{}{
        "node_id":     node.id,
        "node_addr":   node.addr,
        "leader_id":   leaderID,
        "leader_addr": leaderAddr,
    }
    writer.Header().Set("Content-Type", "application/json")

    json.NewEncoder(writer).Encode(out)
}

func main() {
    idFlag := flag.Int("id", 1, "numeric id for this node")
    addressFlag := flag.String("addr", "http://localhost:8001", "public address for this node, used by peers (include scheme and port)")
    peersFlag := flag.String("peers", "", "comma-separated list of peers as id=addr,id=addr")
    flag.Parse()

    peers := make(map[int]string)
    if *peersFlag != "" {
        items := strings.Split(*peersFlag, ",")
        for _, item := range items {
            item = strings.TrimSpace(item)
            if item == "" {
                continue
            }
            parts := strings.SplitN(item, "=", 2)
            if len(parts) != 2 {
                log.Fatalf("bad peer entry: %s", item)
            }
            pid, err := strconv.Atoi(parts[0])
            if err != nil {
                log.Fatalf("bad peer id: %s", parts[0])
            }
            peers[pid] = parts[1]
        }
    }

    peers[*idFlag] = *addressFlag

    node := NewNode(*idFlag, *addressFlag, peers)
    node.startLeaderLoop()

    http.HandleFunc("/cards", func(w http.ResponseWriter, r *http.Request) {
        switch r.Method {
            case "GET":
                node.handleGetCards(w, r)
            case "POST":
                node.handlePostCard(w, r)
            default:
                http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
        }
    })
    http.HandleFunc("/cards/", func(w http.ResponseWriter, r *http.Request) {
        if r.Method == "DELETE" {
            node.handleDeleteCard(w, r)
            return
        }
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
    })
    http.HandleFunc("/replicate", node.handleReplicate)
    http.HandleFunc("/status", node.handleStatus)

    // server expects addr like http://host:port; extract host:port for Listen
    listenAddr := ""
    if strings.HasPrefix(*addressFlag, "http://") {
        listenAddr = strings.TrimPrefix(*addressFlag, "http://")
    } else if strings.HasPrefix(*addressFlag, "https://") {
        listenAddr = strings.TrimPrefix(*addressFlag, "https://")
    } else {
        listenAddr = *addressFlag
    }

    log.Printf("node %d starting on %s, peers=%v, leader=%d (%s)", node.id, listenAddr, peers, node.leaderID, node.leaderAddr)
    if err := http.ListenAndServe(listenAddr, nil); err != nil {
        fmt.Fprintln(os.Stderr, "server error:", err)
        os.Exit(1)
    }
}
