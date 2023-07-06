use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

use crate::board_generator;
use crate::game::{GameState, GuiData};

pub struct Events;

impl Events {
    pub fn process_events(
        game_state: &mut GameState,
        event_pump: &mut EventPump,
        canvas: &mut Canvas<Window>,
        gui_data: &GuiData
    ) -> bool {
        // I leave this part to you, as it's a long piece of code
        for event in event_pump.poll_iter() {
            match event {
                // If the user closes the window or presses the escape key, exit the game
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false;
                }
                Event::Window { win_event: sdl2::event::WindowEvent::Resized(w, h), .. } => {
                    canvas
                        .window_mut()
                        .set_size(w as u32, h as u32)
                        .unwrap();
                }
                // If the user clicks on a square, select that square
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let row = (y as i32 - gui_data.offset) / gui_data.cell_size as i32;
                    let col = (x as i32 - gui_data.offset) / gui_data.cell_size as i32;
                    // Check if the square is within the board
                    if row >= 0 && row < 9 && col >= 0 && col < 9 && x >= gui_data.offset && y >= gui_data.offset {
                        if game_state.board_initialized &&
                           game_state.initial_board[row as usize][col as usize].is_none()
                        {
                            game_state.selected_square = Some((row, col));
                        }
                    }
    
                    // Check if the one of the top row buttons are pressed
                    for index in 0..3 {
                        let button_x_level_1 =
                            gui_data.spacing_level_1 * (index as i32) + 4 * gui_data.offset - 15;
                        if
                            x >= button_x_level_1 &&
                            x <=
                                button_x_level_1 +
                                    (gui_data.button_width_level_1 as i32) +
                                    gui_data.offset &&
                            y >= gui_data.y_level_1 - (gui_data.button_height as i32) - 10 &&
                            y <= gui_data.y_level_1 + (gui_data.button_height as i32) + 10
                        {
                            match index {
                                0 => {
                                    game_state.new_puzzle_button_pressed = true;
                                }
                                1 => {
                                    game_state.candidate_button_pressed =
                                        !game_state.candidate_button_pressed;
                                }
                                2 => {
                                    game_state.solve_button_pressed = true;
                                }
                                _ => {}
                            }
                        }
                    }
    
                    for index in 0..5 {
                        let button_x_level_2 =
                            gui_data.spacing_level_2 * (index as i32) + 2 * gui_data.offset - 15;
                        if
                            x >= button_x_level_2 &&
                            x <=
                                button_x_level_2 +
                                    (gui_data.button_width_level_2 as i32) +
                                    gui_data.offset &&
                            y >= gui_data.y_level_2 - (gui_data.button_height as i32) - 10 &&
                            y <= gui_data.y_level_2 + (gui_data.button_height as i32) + 10
                        {
                            match index {
                                0 => {
                                    game_state.difficulty = board_generator::BoardDifficulty::Beginner;
                                }
                                1 => {
                                    game_state.difficulty = board_generator::BoardDifficulty::Easy;
                                }
                                2 => {
                                    game_state.difficulty = board_generator::BoardDifficulty::Medium;
                                }
                                3 => {
                                    game_state.difficulty = board_generator::BoardDifficulty::Hard;
                                }
                                4 => {
                                    game_state.difficulty = board_generator::BoardDifficulty::Expert;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                // If the user releases the mouse button, check if the new puzzle or solve button is pressed
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                    // Check if the new puzzle button is pressed
                    if game_state.new_puzzle_button_pressed {
                        game_state.selected_square = None;
                        game_state.new_puzzle_button_pressed = false;
    
                        game_state.generate_new_board();
                        game_state.board_initialized = true;
                        game_state.puzzle_solved = false;
                        game_state.invalid_positions = Vec::new();
                        game_state.candidates = vec![vec![None; 9]; 9];
                    }
                    // Check if the solve button is pressed
                    if game_state.solve_button_pressed {
                        game_state.solve_button_pressed = false;
    
                        if game_state.board_initialized {
                            game_state.board = game_state.solved_board.clone();
                            game_state.invalid_positions = Vec::new();
                        }
                    }
                }
                // If the user presses a key, check if it is a number and if so, add it to the board
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    // Check if the key pressed is a number
                    if let Some((x, y)) = game_state.selected_square {
                        match keycode {
                            Keycode::Num1 | Keycode::Kp1 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 1);
                            }
                            Keycode::Num2 | Keycode::Kp2 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 2);
                            }
                            Keycode::Num3 | Keycode::Kp3 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 3);
                            }
                            Keycode::Num4 | Keycode::Kp4 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 4);
                            }
                            Keycode::Num5 | Keycode::Kp5 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 5);
                            }
                            Keycode::Num6 | Keycode::Kp6 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 6);
                            }
                            Keycode::Num7 | Keycode::Kp7 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 7);
                            }
                            Keycode::Num8 | Keycode::Kp8 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 8);
                            }
                            Keycode::Num9 | Keycode::Kp9 => {
                                Self::handle_number_entry(game_state, x as usize, y as usize, 9);
                            }
                            Keycode::Backspace | Keycode::Delete => {
                                if game_state.board_initialized {
                                    game_state.board[x as usize][y as usize] = None;
                                    game_state.invalid_positions.retain(
                                        |&(xi, yi, _)| (xi != x || yi != y)
                                    );
                                }
                                if game_state.candidate_button_pressed {
                                    if let Some(candidates) = &mut game_state.candidates[x as usize][y as usize] {
                                        candidates.clear();
                                    }
                                }
                            }
                            _ => {}
                        }
    
                        // Check if the number entered is valid TODO: Fix this
                        if game_state.board_initialized {
                            if let Some(val) = game_state.board[x as usize][y as usize] {
                                // Check if the coordinates are already in the invalid_positions
                                let mut index = None;
                                for (i, &(xi, yi, _)) in game_state.invalid_positions
                                    .iter()
                                    .enumerate() {
                                    if xi == x && yi == y {
                                        index = Some(i);
                                        break;
                                    }
                                }
    
                                // If the coordinates are in the invalid_positions, check if the value is different.
                                // If it is, remove the tuple
                                if let Some(i) = index {
                                    if game_state.invalid_positions[i].2 != val {
                                        game_state.invalid_positions.remove(i);
                                    }
                                }
    
                                // Check if the move is valid. If it's not, add it to the invalid_positions
                                if
                                    !game_state.is_valid_move(
                                        &game_state.board,
                                        x as usize,
                                        y as usize,
                                        val
                                    )
                                {
                                    game_state.invalid_positions.push((x, y, val));
                                }
                            }
                        }
    
                        // Check if board is complete after enteirng a number
                        if game_state.board == game_state.solved_board {
                            game_state.selected_square = None;
                            game_state.board = vec![vec![None; 9]; 9];
                            game_state.initial_board = vec![vec![None; 9]; 9];
                            game_state.solved_board = vec![vec![None; 9]; 9];
                            game_state.board_initialized = false;
                            game_state.puzzle_solved = true;
                            game_state.invalid_positions = Vec::new();
                        }
                    }
                }
                _ => {}
            }
        }
        return true;
    }

    fn handle_number_entry(game_state: &mut GameState, x: usize, y: usize, val: i32) {
        if game_state.candidate_button_pressed {
            // if the number is already in the candidates, remove it otherwise add it to Vector
            if let Some(candidates) = &mut game_state.candidates[x][y] {
                if candidates.contains(&val) {
                    candidates.retain(|&x| x != val);
                } else {
                    candidates.push(val);
                }
            } else {
                game_state.candidates[x][y] = Some(vec![val]);
            }
        } else {
            game_state.candidates[x][y] = None;
            game_state.board[x][y] = Some(val);
        }
    }
    
}
