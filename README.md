<h1 align="center">Eleventh</h1>

<p align="center">⚽ <em>Only 11 wins</em> 🃏</p>

<p align="center">
    <img src=".project/images/cover.png" alt="Eleventh Cover" width="400" style="max-width:100%;">
</p>

**Eleventh** is a turn-based card game inspired by FIFA’s Ultimate Team cards and the strategic style of Soccer Manager.
Players build their own dream team using collectible cards, manage tactics, and compete in tactical duels against other managers. 


## Notice

This is the 2nd version of this project and this is under maintenance.
Since the 2nd problem of the PBL differs a lot in architecture from the 1st one,
this is needed a new major version with possible breaking changes.

Now, this is possible to use HTTP frameworks, so QuickAPI will be deprecated.
Since the focus now is fault-tolerant system, I'll choose Typescript (Deno) or Elixir
to handle this correctly.

If you need to see the old project, open the `v1.0.0` tag version,
open on Github or do this using git commands.


## PBL Context

This project was developed for the subject TEC502 – Concurrency & Connectivity, which is taught using a Problem-Based Learning (PBL) approach.

In PBL, students work in groups to solve open-ended problems, progressing step by step through research, discussion, and implementation. This project in specific is individual, but sessions are organized in group to share experiences and brainstorming.

Because of this nature, I've created the `.project/` folder that have the sessions summaries, goals and others.


## Architecture

This project follows the craftsmanship principles. This is organized by using the Vertical-Slice, but implementing Clean Architecture + Some DDD Principles.

This was decided to go with microsservices since this probably fits well to this problem and I can scale each service individually as it needs to. Each available service goes into `eleventh/service` and shared code goes on `eleventh/shared`.

Each service has its own REST APIs, RPCs, Repositories and its own algorithms for distributed system. Each of them with their own tests and also running instructions. Reach each documentation for more information.

## Running it

It's possible to run this project in three different ways: locally, locally into containers, distributed across machines into containers. The first one allow us to test it on development. The second one for demonstration and the third one for real systems.

Don't take the application itself too seriously, I am not trying to solve a gaming problem, but a distributed system one. So, the game UI and logic won't be that fun or any great.

## Tooling

This project uses virtual environment managed by Poetry, so make sure you have this installed to run this properly.


## Strategies

### Generating Rewards

- Bully (easier to implement)
- Leader-Follower

### Getting Rewards

- ~~Token Ring~~
- ~~Updates the commits from the previous, and then from itself~~

- Causal consistency  
  - See: https://arxiv.org/pdf/1805.06358

### Managing User's Deck

- Eventually consistent
- Follower Reads for Queries

### Card Trade

- Eventually consistent to add a card
- 2 Phase Commit for confirm the trade

### Match Making

- Leader-Follower
  - The Host is the leader and decides the state of the match
  - The Guest must redirect the user's request and return back to the user
  - Leader-Follower has simple election: the first player to request is the host
- Simpler fault-tollerancy
  - If the server goes down, the match ends
  - The user must open the game in a new server to be able to play again
  - Since this is stateless, there is no recover or consensus process at the end