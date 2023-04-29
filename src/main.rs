extern crate macroquad;
use macroquad::prelude::*;

use game::{SQUARES, ThisRoundMove, GameState, determine_winner, player_move, switch_starting_entity};
use graphics::{window_conf, display_message_screen, render_board, render_shapes};
use ai::ai_move;

mod game;
mod graphics;
mod ai;


#[macroquad::main(window_conf)]
async fn main() {

    // This variable will determine who will make a first move
    let mut starting_entity: ThisRoundMove = ThisRoundMove::PlayerMove;
    let mut game_state: ThisRoundMove = starting_entity.clone();

    let mut game_over = false;

    // 2x2 array to keep track of all positions -> board state
    let mut board_state: [[i8; SQUARES as usize]; SQUARES as usize] = [[0; SQUARES as usize]; SQUARES as usize];

    // Status message that will be displayed
    let mut message: String = String::new();

    // Main game loop
    loop {
        if !game_over {

            // Make a move based on whose turn it is
            match game_state {
                ThisRoundMove::PlayerMove => {
                    message = String::from("Player Move");
                    if is_mouse_button_pressed(MouseButton::Left) {
                        player_move(&mut board_state, &mut game_state);
                    }
                }
                ThisRoundMove::AIMove => {
                    message = String::from("AI Move");
                    ai_move(&mut board_state, &mut game_state);
                }
            }

            // Determine game state
            match determine_winner(&board_state) {
                GameState::PlayerWin => {
                    message = String::from("Player Won!");
                    println!("\n===================================\n");
                    println!("Player Won!");
                    println!("===================================\n");
                    game_over = true;
                },
                GameState::AIWin => {
                    message = String::from("AI Won!");
                    println!("\n===================================\n");
                    println!("AI Won!\n");
                    println!("===================================\n");
                    game_over = true;
                },
                GameState::Draw => {
                    message = String::from("Draw!");
                    println!("\n===================================\n");
                    println!("Draw -> No other possible moves available\n");
                    println!("===================================\n");
                    game_over = true;
                },
                GameState::GameInProgress => {}
            }
        } else {
            display_message_screen(&message);
            // Check if user wants to quit or reset the game
            if is_key_pressed(KeyCode::Q) {
                println!("Q was pressed\nExiting the game loop");
                break
            } else if is_key_pressed(KeyCode::R) {
                // Reset game state
                switch_starting_entity(&mut starting_entity, &mut game_state);
                board_state = [[0; SQUARES as usize]; SQUARES as usize];
                game_over = false;
                println!("R was Pressed");
            }
        }

        // Render everything
        render_board(&message, &board_state);
        render_shapes(&board_state);

        next_frame().await
    }
    println!("Final Board State: {:?}", board_state);
}
