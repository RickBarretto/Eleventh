# Goals

Since this project is a project-based learning project, there are some weekly goals to follow and accomplish 
according to what have been discussed on the previous discussion session.

There are two kind of goals, development - to implement something physical on the labs - and 
session - to implement strategies for the next session to discuss.

Some (most) of them are not included here, but internally on the source code of the project.

## All Goals

### Week 0

**Development**
Proof of Concept: two containers talking to each other (eg.: ping-pong).

See: [Chat - Proof of Concept](./chat/)

**Session**
Create a Docker environment by using `docker compose up` with small images and instructions to run it.

See: [Chat - Proof of Concept](./chat/)

### Week 1

**Development**
Implement a chat system between two clients via a single server.

See: [Chat - Proof of Concept](./chat/)

**Session**
Implement an architectural diagram for the game system, evidencing the interaction client-server and rooms.
Sketch the rules and flow of a simple game that will serve as prototype for testing and communication.

See: [Game Flow - Diagrams](./game-flow/)

### Week 2

**Development**
Implement a way to allow multiple users connected at the same time without major issues.

This goal was implemented since the Python version (by using asyncio) and 
on the current version (Rust) by using Tokio and Mutexes. 
Both versions implemented a HTTP subset protocol with async routes, just like FastAPI does,
due to this fact I named it as QuickAPI.   

**Session**
Bring set of rules and ideas for your own game.

See: [Game Flow - Diagrams](./game-flow/)

### Week 3

Finish it.