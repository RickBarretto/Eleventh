# Eleventh - Week 1 prototype

This is a minimal Elixir prototype for the distributed card game Week 1
deliverable.

Run with Docker Compose to start two nodes:

```pwsh
docker-compose up --build
```

Endpoints:

- POST /join -> {id}
- POST /claim { user_id }
- POST /match { host_id, guest_id }
- POST /trade { from_id, to_id }

```pwsh
curl -X POST http://localhost:4000/join -H \"Content-Type: application/json\"
curl -X POST http://localhost:4000/claim -H \"Content-Type: application/json\" -d '{\"user_id\":\"<ID_FROM_JOIN>\"}'
curl -X POST http://localhost:4000/match -H \"Content-Type: application/json\" -d '{\"host_id\":\"<HOST_ID>\",\"guest_id\":\"<GUEST_ID>\"}'
curl -X POST http://localhost:4000/trade -H \"Content-Type: application/json\" -d '{\"from_id\":\"<FROM>\",\"to_id\":\"<TO>\"}'
```

<h1 align="center">Eleventh</h1>

<p align="center">⚽ <em>Only 11 wins</em> 🃏</p>

<p align="center">
    <img src=".project/images/cover.png" alt="Eleventh Cover" width="400" style="max-width:100%;">
</p>

**Eleventh** is a turn-based card game inspired by FIFA’s Ultimate Team cards
and the strategic style of Soccer Manager. Players build their own dream team
using collectible cards, manage tactics, and compete in tactical duels against
other managers.

## Notice

This is the 2nd version of this project and this is under maintenance. Since the
2nd problem of the PBL differs a lot in architecture from the 1st one, this is
needed a new major version with possible breaking changes.

Now, this is possible to use HTTP frameworks, so QuickAPI will be deprecated.
Since the focus now is fault-tolerant system, I'll choose Typescript (Deno) or
Elixir to handle this correctly.

If you need to see the old project, open the `v1.0.0` tag version, open on
Github or do this using git commands.

## PBL Context

This project was developed for the subject TEC502 – Concurrency & Connectivity,
which is taught using a Problem-Based Learning (PBL) approach.

In PBL, students work in groups to solve open-ended problems, progressing step
by step through research, discussion, and implementation. This project in
specific is individual, but sessions are organized in group to share experiences
and brainstorming.

Because of this nature, I've created the `.project/` folder that have the
sessions summaries, goals and others.
