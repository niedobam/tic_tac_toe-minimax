use macroquad::prelude::*;

use crate::graphics::{UI_OFFSET, SQUARE_SIZE};

// Game size and win condition
pub const SQUARES: u8 = 3;
pub const WIN_CONDITION: u8 = 3;

#[derive(Clone)]
pub enum ThisRoundMove {
    PlayerMove,
    AIMove
}

pub enum GameState {
    PlayerWin,
    AIWin,
    Draw,
    GameInProgress
}

pub fn is_position_available(board_state: &[[i8; SQUARES as usize]; SQUARES as usize], row: &u8, column: &u8) -> bool {
    if board_state[*row as usize][*column as usize] == 0 {
        return true
    }
    return false
}

pub fn get_user_click_position(board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) -> Option<(u8, u8)> {

    // If user clicks on valid positions on the screen
    if mouse_position().0 > screen_width() - UI_OFFSET as f32 {
        return None
    }

    let mut user_click_position: (u8, u8) = (0, 0);

    // Get row
    for i in 1..=SQUARES {
        if mouse_position().0 < i as f32 * SQUARE_SIZE as f32 {
            user_click_position.1 = i as u8;
            break
        }
    }

    // Get column
    for i in 1..=SQUARES {
        if mouse_position().1 < i as f32 * SQUARE_SIZE as f32 {
            user_click_position.0 = i as u8;
            break
        }
    }

    if !is_position_available(board_state, &(user_click_position.0 - 1), &(user_click_position.1 - 1)) {
        return None
    }
    return Some(user_click_position)
}

pub fn player_move(board_state: &mut [[i8; SQUARES as usize]; SQUARES as usize], game_state: &mut ThisRoundMove) {
    let board_copy: [[i8; SQUARES as usize]; SQUARES as usize] = board_state.clone();
    match get_user_click_position(&board_copy) {
        Some(position) => {
            board_state[position.0 as usize - 1][position.1 as usize - 1] = 1;
            *game_state = ThisRoundMove::AIMove
        }
        None => {
            println!("Click on a viable free space on the game board")
        }
    }
}

pub fn determine_winner(board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) -> GameState {

    let mut player_counter: u8;
    let mut ai_counter: u8;

    // Check rows
    for row in 0..SQUARES {
        player_counter = 0;
        ai_counter = 0;
        for column in 0..SQUARES {
            if board_state[row as usize][column as usize] == 1 {
                player_counter += 1;
                ai_counter = 0;
                if player_counter == WIN_CONDITION {
                    return GameState::PlayerWin
                }
            } else if board_state[row as usize][column as usize] == -1 {
                ai_counter += 1;
                player_counter = 0;
                if ai_counter == WIN_CONDITION {
                    return GameState::AIWin
                }
            }
        }
    }

    // Check columns
    for column in 0..SQUARES {
        ai_counter = 0;
        player_counter = 0;
        for row in 0..SQUARES {
            if board_state[row as usize][column as usize] == 1 {
                player_counter += 1;
                ai_counter = 0;
                if player_counter == WIN_CONDITION {
                    return GameState::PlayerWin
                }
            } else if board_state[row as usize][column as usize] == -1 {
                ai_counter += 1;
                player_counter = 0;
                if ai_counter == WIN_CONDITION {
                    return GameState::AIWin
                }
            }
        }
    }

    // Check diagonals
    // From top left to bottom right
    player_counter = 0;
    ai_counter = 0;
    for i in 0..SQUARES {
        if board_state[i as usize][i as usize] == 1 {
            player_counter += 1;
            ai_counter = 0;
            if player_counter == WIN_CONDITION {
                return GameState::PlayerWin
            }
        } else if board_state[i as usize][i as usize] == -1 {
            ai_counter += 1;
            player_counter = 0;
            if ai_counter == WIN_CONDITION {
                return GameState::AIWin
            }
        }
    }

    // From top right to bottom left
    ai_counter = 0;
    player_counter = 0;
    for i in 0..SQUARES {
        if board_state[i as usize][(SQUARES - (i + 1)) as usize] == 1 {
            player_counter += 1;
            ai_counter = 0;
            if player_counter == WIN_CONDITION {
                return GameState::PlayerWin
            }
        } else if board_state[i as usize][(SQUARES - (i + 1)) as usize] == -1 {
            ai_counter += 1;
            player_counter = 0;
            if ai_counter == WIN_CONDITION {
                return GameState::AIWin
            }
        }
    }

    // Check for draw condition else return game running state
    for row in 0..SQUARES {
        for column in 0..SQUARES {
            if board_state[row as usize][column as usize] == 0 {
                return GameState::GameInProgress
            }
        }
    }
    return GameState::Draw
}

// After game reset switches starting player
pub fn switch_starting_entity(starting_entity: &mut ThisRoundMove, game_state: &mut ThisRoundMove) {
    match starting_entity {
        ThisRoundMove::PlayerMove => *starting_entity = ThisRoundMove::AIMove,
        ThisRoundMove::AIMove => *starting_entity = ThisRoundMove::PlayerMove
    }
    *game_state = starting_entity.clone();
}
