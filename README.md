# Oxihex ⬡

Oxihex is a modern evolution of minimalist hex-based strategy games. It aims to eliminate player downtime through **simultaneous seasonal turns** and deepens the gameplay with a dedicated **Diplomacy & Chronicle** system.

## 🛠 Tech Stack
- **Engine:** [Bevy](https://bevyengine.org/) (Targeting WebAssembly & Desktop)
- **Backend:** [Axum](https://github.com/tokio-rs/axum) (High-performance asynchronous Rust) //TODO
- **Networking:** WebSockets with [Bincode](https://github.com/bincode-org/bincode) for ultra-low overhead binary serialization.
- **Shared Logic:** Isomorphic Rust crate for shared game state and protocols.

## 🌟 Key Features
- **Simultaneous Turns:** No more waiting for others to finish their move.
- **Seasons System:** Plan your expansion in Spring and survive the upkeep in Winter.
- **The Chronicle:** A persistent record of world events, betrayals, and alliances written by the players.
- **Vassalage:** Vanquished players may stay in the game as governors, serving under their conqueror.

## 📜 License
This project is licensed under the [MIT License](LICENSE).