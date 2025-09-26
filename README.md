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

## Week 1 - Deno demo

This workspace contains a minimal Deno-based prototype for Week 1: user join,
claim a daily pack, play a match (host always wins 1:0), and trade packs between
users. Servers coordinate via Redis so multiple server instances can be run.

Quick start (Docker Compose):

```powershell
docker compose up --build
# in another shell, seed packs into Redis
docker compose exec server1 deno run --allow-net scripts/seed_packs.ts
```

Try the concurrency claim test (after seeding):

```powershell
deno run --allow-net scripts/claim_test.ts http://localhost:8000
```

API endpoints:

- GET /join -> { id }
- POST /claim { userId } -> { pack }
- POST /match { hostId, guestId } -> publishes match result
- POST /trade { fromUser, toUser, packId } -> transfer pack ownership
