# TicTacToe-minimax


![tic-tac-toe](https://user-images.githubusercontent.com/127039716/235303174-f781f020-28d1-4f0a-8070-aa2826163f27.gif)

## General Information
This was written as a final project for a university course - Algorithms of Artificial Intelligence, at VUT Brno. It is a simple game of Tic-Tac-Toe, where player plays against AI opponent based on [Minimax](https://en.wikipedia.org/wiki/Minimax) algorithm with [Alpha-beta](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning) pruning.

It is written in [Rust](https://www.rust-lang.org/) utilizing [macroquad](https://github.com/not-fl3/macroquad) gaming library.


## Prerequisites
  * Rust development environment. Instructions can be found [here](https://www.rust-lang.org/tools/install)
  
  
  * macroquad dependencies:
    #### Linux
    ```
    # ubuntu system dependencies
    apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

    # fedora system dependencies
    dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

    # arch linux system dependencies
    pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
    ```
    
    Instructions for other platforms can be found [here](https://github.com/not-fl3/macroquad#windows)


## Build Instructions
  ### Clone this repository
  ```bash
  git clone https://github.com/niedobam/TicTacToe-minimax
  ```
  ### Go into repository
  ```bash
  cd TicTacToe-minimax
  ```
  ### Run program
  ```rust
  cargo run
  ```
