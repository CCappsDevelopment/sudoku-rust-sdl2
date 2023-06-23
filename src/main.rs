extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::mouse::MouseButton;
use sdl2::render::TextureQuery;

mod board_generator;

//GameState struct to store all relevant game state information
struct GameState {
    selected_square: Option<(i32, i32)>,
    new_puzzle_button_pressed: bool,
    solve_button_pressed: bool,
    board: Vec<Vec<Option<i32>>>,
    initial_board: Vec<Vec<Option<i32>>>,
    solved_board: Vec<Vec<Option<i32>>>,
    board_initialized: bool,
    puzzle_solved: bool,
    difficulty: board_generator::BoardDifficulty,
    invalid_positions: Vec<(i32, i32)>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            selected_square: None,
            new_puzzle_button_pressed: false,
            solve_button_pressed: false,
            board: vec![vec![None; 9]; 9],
            initial_board: vec![vec![None; 9]; 9],
            solved_board: vec![vec![None; 9]; 9],
            board_initialized: false,
            puzzle_solved: false,
            difficulty: board_generator::BoardDifficulty::Medium,
            invalid_positions: Vec::new(),
        }
    }

    pub fn generate_new_board(&mut self) {
        // create a new board generator
        let mut board_generator = board_generator::BoardGenerator::new(self.difficulty.clone());
        
        // generate a new board
        self.board = board_generator.generate_sudoku();
        self.solved_board = board_generator.get_solved_board().unwrap();
        self.initial_board = self.board.clone();
    }
    
    pub fn is_valid_move(&self, board: &Vec<Vec<Option<i32>>>, row: usize, col: usize, val: i32) -> bool {
        // Check if the value is already in the row
        for i in 0..9 {
            if board[row][i] == Some(val) {
                return false;
            }
        }
        // Check if the value is already in the column
        for i in 0..9 {
            if board[i][col] == Some(val) {
                return false;
            }
        }
        // Check if the value is already in the 3x3 square
        let square_row = row - row % 3;
        let square_col = col - col % 3;
        for i in square_row..square_row + 3 {
            for j in square_col..square_col + 3 {
                if board[i][j] == Some(val) {
                    return false;
                }
            }
        }
        return true;
    }

}

// Handles the logic of initializing SDL2 and creating the window and canvas
fn init_sdl2() -> Result<(sdl2::EventPump, sdl2::render::Canvas<sdl2::video::Window>), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window_width: u32 = 9 * 128 + 2 * 50 + 8;
    let window_height: u32 = 9 * 128 + 50 + 250 + 8;

    let window = video_subsystem.window("Sudoku", window_width, window_height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let event_pump = sdl_context.event_pump()?;

    Ok((event_pump, canvas))
}

fn process_events(game_state: &mut GameState, event_pump: &mut sdl2::EventPump) -> bool {
    // I leave this part to you, as it's a long piece of code
    for event in event_pump.poll_iter() {
        match event {
            // If the user closes the window or presses the escape key, exit the game
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return false;
            },
            // If the user clicks on a square, select that square
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                let row = (y as i32 - 50) / 130;
                let col = (x as i32 - 50) / 130;
                // Check if the square is within the board
                if row >= 0 && row < 9 && col >= 0 && col < 9 {
                    if game_state.board_initialized &&  game_state.initial_board[row as usize][col as usize].is_none() {
                        game_state.selected_square = Some((row, col));
                    }
                } 

                // Check if the new game button is pressed
                if x >= 60 && x <= 460 && y >= 1225 && y <= 1315 {
                    game_state.new_puzzle_button_pressed = true;
                }
                // Check if the solve button is pressed
                if x >= 786 && x <= 1186 && y >= 1225 && y <= 1315 {
                    game_state.solve_button_pressed = true;
                }
                // Check if one of the difficulty buttons is pressed
                if x >= 10 && x <= 243 && y >= 1350 && y <= 1410 {
                    game_state.difficulty = board_generator::BoardDifficulty::Beginner;
                }
                if x >= 260 && x <= 490 && y >= 1350 && y <= 1410 {
                    game_state.difficulty = board_generator::BoardDifficulty::Easy;
                }
                if x >= 510 && x <= 740 && y >= 1350 && y <= 1410 {
                    game_state.difficulty = board_generator::BoardDifficulty::Medium;
                }
                if x >= 760 && x <= 990 && y >= 1350 && y <= 1410 {
                    game_state.difficulty = board_generator::BoardDifficulty::Hard;
                }
                if x >= 1010 && x <= 1240 && y >= 1350 && y <= 1410 {
                    game_state.difficulty = board_generator::BoardDifficulty::Expert;
                }
            },
            // If the user releases the mouse button, check if the new puzzle or solve button is pressed
            Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
                // Check if the new puzzle button is pressed
                if  game_state.new_puzzle_button_pressed {
                    game_state.selected_square = None;
                    game_state.new_puzzle_button_pressed = false;

                    game_state.generate_new_board();
                    game_state.board_initialized = true;
                    game_state.puzzle_solved = false;
                    game_state.invalid_positions = Vec::new();
                }
                // Check if the solve button is pressed
                if  game_state.solve_button_pressed {
                    game_state.solve_button_pressed = false;

                    if  game_state.board_initialized {
                        game_state.board =  game_state.solved_board.clone();
                    }
                }
            },
            // If the user presses a key, check if it is a number and if so, add it to the board
            Event::KeyDown { keycode: Some(keycode), .. } => {
                // Check if the key pressed is a number
                if let Some((x, y)) = game_state.selected_square {
                    match keycode {
                        Keycode::Num1 | Keycode::Kp1 =>  game_state.board[x as usize][y as usize] = Some(1),
                        Keycode::Num2 | Keycode::Kp2 =>  game_state.board[x as usize][y as usize] = Some(2),
                        Keycode::Num3 | Keycode::Kp3 =>  game_state.board[x as usize][y as usize] = Some(3),
                        Keycode::Num4 | Keycode::Kp4 =>  game_state.board[x as usize][y as usize] = Some(4),
                        Keycode::Num5 | Keycode::Kp5 =>  game_state.board[x as usize][y as usize] = Some(5),
                        Keycode::Num6 | Keycode::Kp6 =>  game_state.board[x as usize][y as usize] = Some(6),
                        Keycode::Num7 | Keycode::Kp7 =>  game_state.board[x as usize][y as usize] = Some(7),
                        Keycode::Num8 | Keycode::Kp8 =>  game_state.board[x as usize][y as usize] = Some(8),
                        Keycode::Num9 | Keycode::Kp9 =>  game_state.board[x as usize][y as usize] = Some(9),
                        Keycode::Backspace | Keycode::Delete => {
                            if game_state.board_initialized {
                                game_state.board[x as usize][y as usize] = None
                            }
                        },
                        _ => {}
                    }

                    // Check if the number entered is valid TODO: Fix this
                    if game_state.board_initialized {
                        if let Some(val) = game_state.board[x as usize][y as usize] {
                            if !game_state.is_valid_move(&game_state.board, x as usize, y as usize, val) {
                                println!("Invalid move!");
                                game_state.invalid_positions.push((x, y));
                            }
                        }
                    }

                    // Check if board is complete after enteirng a number
                    if game_state.board == game_state.solved_board {
                        println!("You solved the puzzle!");
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

fn draw_board(game_state: &GameState, mut canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: (&sdl2::ttf::Font, &sdl2::ttf::Font, &sdl2::ttf::Font)) -> Result<(), String>{
    // Set the background color and clear the screen
    canvas.set_draw_color(Color::RGB(245, 242, 232));
    canvas.clear();

    // Draw the board and the numbers
    canvas.set_draw_color(Color::RGB(26, 28, 26));
    for i in 0..9 {
        for j in 0..9 {
            let x = j * 128 + 50;
            let y = i * 128 + 50;
            let rect = Rect::new(x as i32, y as i32, 128, 128);
            if Some((i, j)) == game_state.selected_square {
                canvas.draw_rect(rect)?;
                let selected_rect = Rect::new(x+1 as i32, y+1 as i32, 126, 126);
                canvas.set_draw_color(Color::RGB(243,206,161));
                canvas.fill_rect(selected_rect)?;
            } else {
                if game_state.initial_board[i as usize][j as usize].is_some() {
                    canvas.set_draw_color(Color::RGB(225, 223, 216));
                    canvas.fill_rect(rect)?;
                }
                canvas.set_draw_color(Color::RGB(26, 28, 26));
                canvas.draw_rect(rect)?;
            }
        }
    }

    // Drawing thicker lines for the Sudoku's 3x3 grid
    for i in 0..4 {
        let x = i * 128 * 3 + 50;
        let y = i * 128 * 3 + 50;
        let line = Rect::new(x as i32, 50, 3, 1155);
        let column = Rect::new(50, y as i32, 1155, 3);
        canvas.set_draw_color(Color::RGB(26, 28, 26));
        canvas.fill_rect(line)?;
        canvas.fill_rect(column)?;
    }

    if game_state.board_initialized {
        // Draw invalid positions
        for (x, y) in &game_state.invalid_positions {
            let rect_x = y * 128 + 50;
            let rect_y = x * 128 + 50;
            let rect = Rect::new(rect_x as i32, rect_y as i32, 126, 126);
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.draw_rect(rect)?;
        }

        // Draw the numbers
        for i in 0..9 {
            for j in 0..9 {
                let x = j * 128 + 50;
                let y = i * 128 + 50;
                if let Some(val) = game_state.board[i as usize][j as usize] {
                    let surface = font.1.render(&val.to_string())
                        .blended(Color::RGB(0, 0, 0))
                        .map_err(|e| e.to_string())?;
                    let texture_creator = canvas.texture_creator();
                    let texture = texture_creator.create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;
                    let TextureQuery { width, height, .. } = texture.query();
                    let target = Rect::new(
                        x as i32 + (128 - width as i32) / 2,
                        y as i32 + (128 - height as i32) / 2,
                        width,
                        height,
                    );
                    canvas.copy(&texture, None, Some(target))?;
                }
            }
        } 
    }  
    else if !game_state.board_initialized && game_state.puzzle_solved {
        display_gameover_message(&mut canvas, &font.2)?;
    }         

    // Draw the buttons
    draw_button(game_state, &mut canvas, &font.0, 75, 1250, 370, 50, "New Puzzle", Some(game_state.new_puzzle_button_pressed), None)?;
    draw_button(game_state, &mut canvas, &font.0, 75 + (128 * 3) + 342, 1250, 370, 50, "Solve", Some(game_state.solve_button_pressed), None)?;
    draw_button(game_state, &mut canvas, &font.0, 25, 1375, 200, 25, "Beginner", None, Some(board_generator::BoardDifficulty::Beginner))?;
    draw_button(game_state, &mut canvas, &font.0, 25 + 200 + 50, 1375, 200, 25, "Easy", None,Some(board_generator::BoardDifficulty::Easy))?;
    draw_button(game_state, &mut canvas, &font.0, 25 + (200 * 2) + 100, 1375, 200, 25, "Medium", None, Some(board_generator::BoardDifficulty::Medium))?;
    draw_button(game_state, &mut canvas, &font.0, 25 + (200 * 3) + 150, 1375, 200, 25, "Hard", None, Some(board_generator::BoardDifficulty::Hard))?;
    draw_button(game_state, &mut canvas, &font.0, 25 + (200 * 4) + 200, 1375, 200, 25, "Expert", None, Some(board_generator::BoardDifficulty::Expert))?;

    // Present the canvas
    canvas.present();

    Ok(())
}

fn draw_button(
    game_state: &GameState,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    font: &sdl2::ttf::Font,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    text: &str,
    button_pressed: Option<bool>,
    difficulty: Option<board_generator::BoardDifficulty>,
) -> Result<(), String> {
    // Set the button color based on its pressed state
    let button_color = 
        if button_pressed  == Some(true) || difficulty == Some(game_state.difficulty) {
            Color::RGB(243, 206, 161)
        }
        else {
            Color::RGB(245, 242, 232)
        };

    let border_color = Color::RGB(26, 28, 26);

    let x_offset: u32 = 30;
    let y_offset: u32 = 30;
    
    let border_rect = Rect::new(
        x - ((x_offset as i32 + 10) / 2), 
        y - 25, 
        width + x_offset + 15, 
        height + y_offset + 15
    );
    let button_rect = Rect::new(
        x - (x_offset as i32 / 2), 
        y - 20, 
        width + x_offset,
        height + y_offset
    );
    
    // Draw the button border
    canvas.set_draw_color(border_color);
    canvas.fill_rect(border_rect)?;
    // Fill the button with the button color
    canvas.set_draw_color(button_color);
    canvas.fill_rect(button_rect)?;

    // Render and draw the button text
    let surface = font
        .render(text)
        .blended(Color::RGB(0, 0, 0))
        .map_err(|e| e.to_string())?;
    
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width: texture_width, height: texture_height, .. } = texture.query();
    
    let target = Rect::new(
        button_rect.x() + ((button_rect.width() as i32 - texture_width as i32) / 2),
        button_rect.y() + ((button_rect.height() as i32 - texture_height as i32) / 2),
        texture_width, 
        texture_height
    );
    canvas.copy(&texture, None, Some(target))?;
    
    Ok(())
}

fn display_gameover_message(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, font: &sdl2::ttf::Font) -> Result<(), String> {
    // Render and draw the game over message
    let surface = font
        .render("You solved the puzzle!")
        .blended(Color::RGB(26, 28, 26))
        .map_err(|e| e.to_string())?;
    
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    let TextureQuery { width: texture_width, height: texture_height, .. } = texture.query();
    
    let x = 50 + ((128 * 9) as i32 - texture_width as i32) / 2;
    let y = 50 + ((128 * 9) as i32 - texture_height as i32) / 2;
    let x_offset: u32 = 30;
    let y_offset: u32 = 30;
    let target = Rect::new(x, y, texture_width, texture_height);
    let bg_rect = Rect::new(x - (x_offset as i32 / 2), y - 20, texture_width + x_offset, texture_height + y_offset);
    let border_rect = Rect::new(x - ((x_offset as i32 + 10) / 2), y - 25, texture_width + x_offset + 20, texture_height + y_offset + 20);

    canvas.set_draw_color(Color::RGB(26, 28, 26));
    canvas.fill_rect(border_rect)?;
    canvas.set_draw_color(Color::RGB(245, 242, 232));
    canvas.fill_rect(bg_rect)?;

    canvas.copy(&texture, None, Some(target))?;

    Ok(())
}

fn main() -> Result<(), String> {
    let (mut event_pump, mut canvas) = init_sdl2()?;

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let button_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Medium.ttf", 36)?;
    let numbers_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Bold.ttf", 44)?;
    let message_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Bold.ttf", 72)?;

    let mut game_state = GameState::new();

    'running: loop {
        if !process_events(&mut game_state, &mut event_pump) {
            break 'running;
        }

        let _ = draw_board(&game_state, &mut canvas, (&button_font, &numbers_font, &message_font));

        // Limit the frame rate to 60 FPS
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
