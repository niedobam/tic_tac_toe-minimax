use crate::game::{SQUARES, GameState, ThisRoundMove, determine_winner, is_position_available};


pub fn ai_move(board_state: &mut [[i8; SQUARES as usize]; SQUARES as usize], game_state: &mut ThisRoundMove) {
    let best_move: (u8, u8) = find_best_move(board_state);
    board_state[best_move.0 as usize][best_move.1 as usize] = -1;
    *game_state = ThisRoundMove::PlayerMove;
}

// PlayerWin, AIWin or Draw should end the minimax search
pub fn is_terminal_state(board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) -> bool {
    match determine_winner(&board_state) {
        GameState::GameInProgress => false,
        _ => true
    }
}

// Helper function for determining the best possible move for the AI
pub fn find_best_move(board_state: &mut [[i8; SQUARES as usize]; SQUARES as usize]) -> (u8, u8) {
    let mut best_score: i32 = i32::MIN;
    let mut best_move: (u8, u8) = (0, 0);

    // Loop through all possible positions
    for row in 0..SQUARES {
        for column in 0..SQUARES {
            if !is_position_available(board_state, &row, &column) {
                continue
            }

            board_state[row as usize][column as usize] = -1;
            let score: i32 = minimax(board_state, false, 0, i32::MIN, i32::MAX);
            board_state[row as usize][column as usize] = 0;
            if score > best_score {
                best_score = score;
                best_move = (row as u8, column as u8);
            }
        }
    }

    return best_move
}

// MiniMax algorith with AlphaBeta pruning
pub fn minimax(board_state: &mut [[i8; SQUARES as usize]; SQUARES as usize], maximizing_player: bool, number_of_moves: i32, mut alpha: i32, mut beta: i32) -> i32 {

    // Static evaluation of a given position
    if is_terminal_state(board_state) {
        match determine_winner(board_state) {
            GameState::AIWin => return 10 - number_of_moves / 2,
            GameState::PlayerWin => return -10 + number_of_moves / 2,
            GameState::Draw => return 0,
            GameState::GameInProgress => {}
        }
    }

    if maximizing_player {
        let mut best_score: i32 = i32::MIN;
        for row in 0..SQUARES {
            for column in 0..SQUARES {
                if !is_position_available(board_state, &row, &column) {
                    continue
                }

                board_state[row as usize][column as usize] = -1;
                let score: i32 = minimax(board_state, false, number_of_moves + 1, alpha, beta);
                board_state[row as usize][column as usize] = 0;

                if score > best_score {
                    best_score = score;
                }

                if score > alpha {
                    alpha = score;
                }

                if beta <= alpha {
                    break
                }
            }
        }

        return best_score

    } else {
        let mut best_score: i32 = i32::MAX;
        for row in 0..SQUARES {
            for column in 0..SQUARES {
                if !is_position_available(board_state, &row, &column) {
                    continue
                }

                board_state[row as usize][column as usize] = 1;
                let score: i32 = minimax(board_state, true, number_of_moves + 1, alpha, beta);
                board_state[row as usize][column as usize] = 0;

                if score < best_score {
                    best_score = score;
                }

                if score < beta {
                    beta = score;
                }

                if beta <= alpha {
                    break
                }
            }
        }

        return best_score
    }
}
