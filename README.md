# OXIHEX

**OXIHEX** is a high-performance, modular logistics and factory simulation game built on a hexagonal grid. Developed purely in **Rust**, the project utilizes a Data-Driven Architecture (ECS) and is designed from the ground up for massive multiplayer over QUIC, strict logic separation, and moddability.

## 🚀 Key Features

* **Hex-Axial Grid System:** Utilizes axial coordinates `(q, r)` for mathematically clean distances, pathfinding, and neighbor resolution.
* **Chunk-Based World:** The world is partitioned into 32x32 rhomboidal chunks, allowing for practically infinite, memory-efficient map generation.
* **Strict Layer Separation:** Game logic (`domain`), simulation (`sim`), and engine specifics (`client`/`server`) are strictly isolated in a modern Rust workspace.
* **High-Performance Networking:** Built for QUIC, utilizing a custom delta-sync approach for seamless multiplayer logistics.
* **Procedural Generation:** Seed-based deterministic world generation running independently of the game simulation.

## 🏗️ Workspace Architecture

The project is structured as a standard Rust Cargo Workspace, dividing executables (`apps`) from logic libraries (`crates`).

### 🎮 Apps (Executables)
* `client-bin`: The actual game client executable. Ties together the engine (Bevy), the renderer, and network client.
* `server-cli`: The headless, dedicated server executable. Runs the simulation and network authority without graphic overhead.

### 🧩 Crates (Libraries)
* **`domain`**: The absolute core of the game. Contains pure data structures, hex math, chunk definitions, and building prototypes. **Zero engine dependencies.**
* **`sim`**: The game simulation logic (ticks, logistic flows, machine processing). Designed to run deterministically on both server and client (for prediction).
* **`client`**: The Bevy engine integration. Handles ECS systems, rendering, input, UI, and asset loading.
* **`server`**: Server-side orchestration, player session management, and authoritative state handling.
* **`net`**: Shared QUIC networking logic, packet definitions, and serialization/deserialization.
* **`storage`**: Disk I/O operations. Handles saving/loading chunks, player profiles, and world states (e.g., SQLite/RocksDB/File IO).
* **`worldgen`**: Procedural generation algorithms. Takes a seed and outputs populated chunks (terrain, ores, environment).

## 🛠️ Technical Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Engine:** [Bevy](https://bevyengine.org/)
* **Networking:** QUIC protocol
* **Serialization:** Serde + RON (Rusty Object Notation) for game data/modding.

## 📦 Getting Started

### Prerequisites
* Latest stable Rust toolchain.
* (Optional but recommended) LLD linker for faster compile times.

### Running the Project

**Run the game client:**
```bash
cargo run --bin client-bin
```

**Run the headless server:**

```Bash
cargo run --bin server-cli
```