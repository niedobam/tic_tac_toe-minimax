use macroquad::prelude::*;

use crate::game::{SQUARES, WIN_CONDITION};
use crate::ai::is_terminal_state;

// Constants modifying graphics window parameters
pub const SQUARE_SIZE: u16 = 250;
pub const UI_OFFSET: u16 = 400;
pub const CIRCLE_RADIUS: u16 = SQUARE_SIZE / 2 - 40;
pub const WINDOW_SIZE: u32 = SQUARES as u32 * SQUARE_SIZE as u32;

// Helper struct for displaying winner line through circles
#[derive(Debug)]
pub struct WinnerLineIndices {
    point1: (u8, u8),
    point2: (u8, u8),
    main_diagonal: bool,
    second_diagonal: bool,
    vertical: bool
}

// Graphics window settings
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Tic-Tac-Toe".to_owned(),
        window_width: WINDOW_SIZE as i32 + UI_OFFSET as i32,
        window_height: WINDOW_SIZE as i32,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn render_board(current_player: &str, board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) {

    // Set window background color to lightgray
    clear_background(LIGHTGRAY);

    // Draw board border
    draw_rectangle_lines(0., 0., screen_width() - UI_OFFSET as f32, screen_height(), 20., BLACK);

    // Draw vertical lines
    for i in 1..SQUARES {
        draw_line(0. + SQUARE_SIZE as f32 * i as f32,
                  0.,
                  0. + SQUARE_SIZE as f32 * i as f32,
                  screen_height(),
                  2.,
                  BLACK
                 );
    }

    // Draw horizontal lines
    for i in 1..SQUARES {
        draw_line(0.,
                  0. + SQUARE_SIZE as f32 * i as f32,
                  screen_width() - UI_OFFSET as f32,
                  0. + SQUARE_SIZE as f32 * i as f32,
                  2.,
                  BLACK
                 );
    }


    // Game UI section
    let font_size1 = 80.;
    let message1: &str = "Tic-Tac-Toe";
    let text_size1 = measure_text(message1, None, font_size1 as _, 1.0);
    draw_text(
        message1,
        screen_width() - UI_OFFSET as f32 / 2. - text_size1.width / 2.,
        text_size1.height + 20.,
        font_size1,
        DARKGREEN,
        );

    draw_line(
        screen_width() - UI_OFFSET as f32 / 2. - text_size1.width / 2.,
        text_size1.height + 30.,
        screen_width() - ((UI_OFFSET as f32 - text_size1.width) / 2.),
        text_size1.height + 30.,
        5.,
        DARKGREEN
        );

    let message2: &str = "Game State:";
    let font_size2 = 50.;
    draw_text(
        message2,
        screen_width() - UI_OFFSET as f32 / 2. - text_size1.width / 2.,
        screen_height() - 530.,
        font_size2,
        BLACK,
        );

    let font_size3 = 70.;
    match current_player {
        "Player Move" => {
            let text_size3 = measure_text(current_player, None, font_size3 as _, 1.0);
            draw_text(
                current_player,
                screen_width() - UI_OFFSET as f32 / 2. - text_size3.width / 2.,
                screen_height() - 450.,
                font_size3,
                RED,
                );
        },
        "AI Move" => {
            let text_size3 = measure_text(current_player, None, font_size3 as _, 1.0);
            draw_text(
                current_player,
                screen_width() - UI_OFFSET as f32 / 2. - text_size3.width / 2.,
                screen_height() - 450.,
                font_size3,
                BLUE,
                );
        },
        "Draw!" => {
            let text_size3 = measure_text(current_player, None, font_size3 as _, 1.0);
            draw_text(
                current_player,
                screen_width() - UI_OFFSET as f32 / 2. - text_size3.width / 2.,
                screen_height() - 450.,
                font_size3,
                DARKPURPLE,
                );
        },
        "Player Won!" => {
            let text_size3 = measure_text(current_player, None, font_size3 as _, 1.0);
            draw_text(
                current_player,
                screen_width() - UI_OFFSET as f32 / 2. - text_size3.width / 2.,
                screen_height() - 450.,
                font_size3,
                RED,
                );
        },
        "AI Won!" => {
            let text_size3 = measure_text(current_player, None, font_size3 as _, 1.0);
            draw_text(
                current_player,
                screen_width() - UI_OFFSET as f32 / 2. - text_size3.width / 2.,
                screen_height() - 450.,
                font_size3,
                BLUE,
                );
        },
        _ => {}
    }

    let message4: &str = "Instructions:";
    let font_size4 = 50.;
    draw_text(
        message4,
        screen_width() - UI_OFFSET as f32 / 2. - text_size1.width / 2.,
        screen_height() - 250.,
        font_size4,
        BLACK,
        );

    match is_terminal_state(board_state) {
        false => {
            let message5: &str = "Click on the Board";
            let font_size5 = 40.;
            let text_size5 = measure_text(message5, None, font_size5 as _, 1.0);
            draw_text(
                message5,
                screen_width() - UI_OFFSET as f32 + (UI_OFFSET as f32 - text_size5.width) / 2.,
                screen_height() - 190.,
                font_size5,
                DARKGREEN,
                );
        },
        true => {
            let message6: &str = "Press [R] to Restart";
            let font_size6 = 40.;
            let text_size6 = measure_text(message6, None, font_size6 as _, 1.0);


            draw_text(
                message6,
                screen_width() - UI_OFFSET as f32 + (UI_OFFSET as f32 - text_size6.width) / 2.,
                screen_height() - 190.,
                font_size6,
                DARKGREEN,
                );
            let message7: &str = "Press [Q] to Quit";
            let font_size7 = 40.;
            let text_size7 = measure_text(message6, None, font_size6 as _, 1.0);
            draw_text(
                message7,
                screen_width() - UI_OFFSET as f32 + (UI_OFFSET as f32 - text_size7.width) / 2.,
                screen_height() - 140.,
                font_size7,
                DARKGREEN,
                );
        }
    }
}

// Message screen settings
pub fn display_message_screen(message: &String) {
    let font_size = 60.;

    draw_text(
        message,
        screen_width() - UI_OFFSET as f32,
        screen_height() - UI_OFFSET as f32,
        font_size,
        BLACK,
        );
}

// Draws circles on the board based on AI and player moves
pub fn render_shapes(board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) {
    for row in 0..SQUARES {
        for column in 0..SQUARES {
            if board_state[row as usize][column as usize] == 1 {
                let x_circle_position = (1. + column as f32) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.;
                let y_circle_position = (1. + row as f32) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.;
                draw_circle(x_circle_position,
                            y_circle_position,
                            CIRCLE_RADIUS as f32,
                            RED
                           );
            }

            if board_state[row as usize][column as usize] == -1 {
                let x_circle_position = (1. + column as f32) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.;
                let y_circle_position = (1. + row as f32) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.;
                draw_circle(x_circle_position,
                            y_circle_position,
                            CIRCLE_RADIUS as f32,
                            BLUE
                           );
            }
        }
    }

    match get_winner_line_indices(board_state) {
        Some(line_points) => {
            if line_points.main_diagonal {
                draw_line(
                    (line_points.point1.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    (line_points.point1.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    (line_points.point2.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    (line_points.point2.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    8.,
                    BLACK
                    );
            } else if line_points.second_diagonal {
                draw_line(
                    (line_points.point1.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    (line_points.point1.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    (line_points.point2.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    (line_points.point2.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    8.,
                    BLACK
                    );
            } else if line_points.vertical {
                draw_line(
                    (line_points.point1.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.,
                    (line_points.point1.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    (line_points.point2.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.,
                    (line_points.point2.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    8.,
                    BLACK
                    );
            } else {
                draw_line(
                    (line_points.point1.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. - CIRCLE_RADIUS as f32 - 20.,
                    (line_points.point1.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.,
                    (line_points.point2.1 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2. + CIRCLE_RADIUS as f32 + 20.,
                    (line_points.point2.0 as f32 + 1.) * SQUARE_SIZE as f32 - SQUARE_SIZE as f32 / 2.,
                    8.,
                    BLACK
                    );
            }
        },
        None => {}
    }
}

// Helper function for displaying line through winner positions
fn get_winner_line_indices(board_state: &[[i8; SQUARES as usize]; SQUARES as usize]) -> Option<WinnerLineIndices> {

    let mut final_indices: WinnerLineIndices = WinnerLineIndices {
        point1: (0, 0),
        point2: (0, 0),
        main_diagonal: false,
        second_diagonal: false,
        vertical: false
    };

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
                if column == 0 {
                    final_indices.point1 = (row, column);
                } else if column == SQUARES - 1 {
                    final_indices.point2 = (row, column);
                }
                if player_counter == WIN_CONDITION {
                    return Some(final_indices)
                }
            } else if board_state[row as usize][column as usize] == -1 {
                ai_counter += 1;
                player_counter = 0;
                if column == 0 {
                    final_indices.point1 = (row, column);
                } else if column == SQUARES - 1 {
                    final_indices.point2 = (row, column);
                }
                if ai_counter == WIN_CONDITION {
                    return Some(final_indices)
                }
            }
        }
    }

    // Check columns
    final_indices.point1 = (0, 0);
    final_indices.point2 = (0, 0);
    for column in 0..SQUARES {
        ai_counter = 0;
        player_counter = 0;
        for row in 0..SQUARES {
            if board_state[row as usize][column as usize] == 1 {
                player_counter += 1;
                ai_counter = 0;
                if row == 0 {
                    final_indices.point1 = (row, column);
                } else if row == SQUARES - 1 {
                    final_indices.point2 = (row, column);
                }
                if player_counter == WIN_CONDITION {
                    final_indices.vertical = true;
                    return Some(final_indices)
                }
            } else if board_state[row as usize][column as usize] == -1 {
                ai_counter += 1;
                player_counter = 0;
                if row == 0 {
                    final_indices.point1 = (row, column);
                } else if row == SQUARES - 1 {
                    final_indices.point2 = (row, column);
                }
                if ai_counter == WIN_CONDITION {
                    final_indices.vertical = true;
                    return Some(final_indices)
                }
            }
        }
    }

    // Check diagonals
    // From top left to bottom right
    final_indices.point1 = (0, 0);
    final_indices.point2 = (0, 0);
    player_counter = 0;
    ai_counter = 0;
    for i in 0..SQUARES {
        if board_state[i as usize][i as usize] == 1 {
            player_counter += 1;
            ai_counter = 0;
            if i == 0 {
                final_indices.point1 = (i, i);
            } else if i == SQUARES - 1 {
                final_indices.point2 = (i, i);
            }
            if player_counter == WIN_CONDITION {
                final_indices.main_diagonal = true;
                return Some(final_indices)
            }
        } else if board_state[i as usize][i as usize] == -1 {
            ai_counter += 1;
            player_counter = 0;
            if i == 0 {
                final_indices.point1 = (i, i);
            } else if i == SQUARES - 1 {
                final_indices.point2 = (i, i);
            }
            if ai_counter == WIN_CONDITION {
                final_indices.main_diagonal = true;
                return Some(final_indices)
            }
        }
    }

    // From top right to bottom left
    final_indices.point1 = (0, 0);
    final_indices.point2 = (0, 0);
    ai_counter = 0;
    player_counter = 0;
    for i in 0..SQUARES {
        if board_state[i as usize][(SQUARES - (i + 1)) as usize] == 1 {
            player_counter += 1;
            ai_counter = 0;
            if i == 0 {
                final_indices.point2 = (i, (SQUARES - (i + 1)));
            } else if i == SQUARES - 1 {
                final_indices.point1 = (i, (SQUARES - (i + 1)));
            }
            if player_counter == WIN_CONDITION {
                final_indices.second_diagonal = true;
                return Some(final_indices)
            }
        } else if board_state[i as usize][(SQUARES - (i + 1)) as usize] == -1 {
            ai_counter += 1;
            player_counter = 0;
            if i == 0 {
                final_indices.point2 = (i, (SQUARES - (i + 1)));
            } else if i == SQUARES - 1 {
                final_indices.point1 = (i, (SQUARES - (i + 1)));
            }
            if ai_counter == WIN_CONDITION {
                final_indices.second_diagonal = true;
                return Some(final_indices)
            }
        }
    }
    return None
}
