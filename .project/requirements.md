# Problem 1: Multiplayer Card Game

## Context

* The online games market is growing, with a focus on multiplayer experiences.
* Centralized servers allow:
  * Data persistence
  * Management of complex states
  * Continuous updates
* Low-latency communication is essential for a smooth gameplay experience.

## Problem

The startup must develop an **online multiplayer card game** with:

* A centralized server for logic and communication.
* **Bidirectional, real-time communication**.
* Features:
  1. Simultaneous connection of multiple players.
  2. Display of communication latency.
  3. **1v1 matches** (no multiple pairings for the same player).
  4. Card pack system (global stock):
     * Fair distribution
     * Prevent duplication or card loss
* Automatic stress tests to ensure fairness and server performance.

## Restrictions

1. Use **Docker containers** for development and testing.
2. **No communication frameworks** allowed.
   * Only **native system sockets**.

## Rules

* Individual project.
* **Final deadline:** 16/09/2025.
* Deliverables:
  * Source code on **GitHub** (with README and test scripts).
  * Report (maximum 8 pages, SBC format).
* Presentation:
  * At LARSID Lab.
  * 25 minutes per student (system demo + technical questions).

## Notes

* Late submission: -20% plus -5% per day (within the same week).
* Plagiarism or identical work: **grade zero**.
* Problem details may be updated during the course.

## Evaluation

* Tutorial performance → **30%**
* Product report (PDF) → **20%**
* Product on GitHub → **50%**
