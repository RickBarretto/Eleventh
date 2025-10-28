# PBL Session Summaries

## Week 1 — 14/08/2025

**Facts**

* The choice of game is free, with a focus on 1×1 duels (e.g., checkers).
* Use of Docker for environment containerization.
* Frameworks are allowed (except for communication, which must use native sockets).

**Ideas**

* Structure of multiple rooms, each with 2 players.
* Pairing modes: random or by invitation/code.
* Card system for gameplay (using mutex for concurrency).
* Checkers as a prototype to validate communication.
* Avoid microservices, prioritizing a simpler solution.

**Questions**

* Concurrency: where should mutual exclusion be applied and how to prove absence of race conditions?
* Transport protocols and message format with native sockets.
* Minimum level of interface (CLI or simple windows).
* Use of native socket library (blocking, timeout, multiplexing).
* Which programming language best facilitates the use of native sockets.

**Goals**

* Study and apply Docker.
* Prove communication between two containers using sockets.

---

## Week 2 — 19/08/2025

**Facts**

* No facts were defined.

**Ideas**

* Use of *goroutines* (Go) to handle multiple concurrent clients.
* Asynchronous programming to manage simultaneous connections.
* Interface suggestions: Charmy (Go), Ncurses, Rich (Python), Web.
* Use of WebSockets.
* Data persistence for login and saving cards.

**Questions**

* Which transport and application protocols are most suitable for each part of the game?

**Goals**

* Create an architecture diagram for client-server interaction and rooms.
* Outline rules and flow of a simple game as a prototype.
* Implement a chat system between two clients connected via the server.

---

## Week 3 — 26/08/2025

**Facts**

* No facts were defined.

**Ideas**

* Data persistence structure.
* Token-based user verification.
* Automated login using JSON/TXT file.
* Use of *Make*.
* Receive loop: if it ends, close connection.
* Open connection only to send a request, then close to avoid too many threads.
* Load balancing.

**Questions**

* How to detect if a client disconnects from the server?
* Is it viable and efficient to keep each player in a dedicated thread on the server?
* How does a server actually work?

**Goals**

* Provide a description and rules of the proposed game.
* Implement a structure that supports multiple users connected to the server without issues.

---

## Week 4 — 02/09/2025

**Facts**

* No facts were defined.

**Ideas**

* Use of WebSockets for the web interface.
* Use of “/command” for chat-based commands.
* State machine for client to wait for server responses.
* Client with 2 threads: one for player I/O, another for server I/O.

**Questions**

* No questions were defined.

**Goals**

* Finish the game.

