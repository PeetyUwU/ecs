# Entity Component System for Moba Game

The server for the MOBA game is responsible for simulating the game world for multiple players. It processes player inputs, manages game state updates, and ensures synchronization across all connected clients. The server must handle concurrent player actions, resolve conflicts, and maintain a consistent and responsive game environment.

## Table of Contents

-   [Entity Component System for Moba Game](#entity-component-system-for-moba-game)
    -   [Table of Contents](#table-of-contents)
    -   [1. Goal](#1-goal)
    -   [2. Inputs](#2-inputs)
    -   [3. Outputs](#3-outputs)
    -   [4. Steps](#4-steps)
    -   [5. Unknowns](#5-unknowns)
    -   [6. Edge Cases](#6-edge-cases)
    -   [7. Definition of Done](#7-definition-of-done)
    -   [8. Notes](#8-notes)

---

## 1. Goal

Build a backend app that takes input from player (e.g. via network manager) and simulates what happens in a "world" and returns the state of the "world".

---

## 2. Inputs

-   Network messages (e.g. player actions, chat messages)
-   Game state synchronization requests
-   Player authentication tokens
-   Latency or ping data

---

## 3. Outputs

-   Game state with data for each player separately
-   Updated world state after processing player actions
-   Notifications or events triggered by game logic
-   Synchronization data for all connected clients

---

## 4. Steps

1. World
2. Resources manager (global data)
3. Component manager (per entity data)
4. Entities
5. Components
6. Query for components
7. Events
8. Shedules
9. Systems

---

## 5. Unknowns

-   How to save components for fast iter
-   How to implement Query system for X components

---

## 6. Edge Cases

-   Player disconnects
-   Entity takes lethal damage and heals in the same frame
-   Entity out of bounds
-   Complete crash
-   Both bases die in the same frame
-   Anything happens in the same frame

---

## 7. Definition of Done

-   The server can process player inputs and update the game state correctly.
-   All edge cases are handled, and the game remains stable under stress.
-   The game state is synchronized across all clients with minimal latency.
-   The system is scalable and can handle the expected number of concurrent players.
-   Comprehensive tests are written and pass successfully.
-   Documentation is complete and explains the system architecture and usage.

---

## 8. Notes
