# CardGames

A simple 2D card game with isometric view built with Rust and Macroquad. This project was created to explore game development concepts in Rust and gain familiarity with Macroquad for rendering and game loop management.

> NOTE: The name is plural because I possibly intented to make multiple small games. Will possibly be renamed

Gif showing the current state of the game:
![demo](/pictures/cardgames_demo.gif)

## Table of content:

-   [Installation](#installation)
-   [Usage](#usage)
-   [Project Structure](#project-structure)

## Installation

1. **Install [Rust](https://www.rust-lang.org/fr/tools/install)**

2. **Clone the repo:**

```sh
git clone https://github.com/Gaspard-Savoureux/CardGame.git
cd rust-card-game-macroquad
```

3. **Dependencies**: Will be automatically installed when running cargo. To consult the dependencies, see [Cargo.toml](/Cargo.toml).

## Usage

1. **Build & Run**:

```sh
cargo run
```

2. **Gameplay**: Available soon.

3. **Configuration**: Available soon.

## Project Structure

> I used the [file-tree-generator](https://marketplace.visualstudio.com/items?itemName=Shinotatwu-DS.file-tree-generator) extension to make the following structure.

```
📦src
 ┣ 📂game
 ┃ ┣ 📜card.rs
 ┃ ┣ 📜deck.rs
 ┃ ┣ 📜effect.rs
 ┃ ┣ 📜hand.rs
 ┃ ┣ 📜isometric_manipulation.rs
 ┃ ┣ 📜keymapping.rs
 ┃ ┣ 📜life.rs
 ┃ ┣ 📜mod.rs
 ┃ ┣ 📜player.rs
 ┃ ┗ 📜ui.rs
 ┗ 📜main.rs
```

-   **main.rs**: Entry point. Initializes the game loop using Macroquad.
-   **keymapping.rs**: Input handling.
-   **ui.rs**: Show general ui elements (will possibly be renamed).
-   **TO COMPLETE**

### Source for the assets in the [assets](/assets/) folder:

**Tileset:** https://scrabling.itch.io/pixel-isometric-tiles
