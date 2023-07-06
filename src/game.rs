extern crate sdl2;

use std::time::Duration;

use events::Events;
use screen_renderer::ScreenRenderer;

use crate::{events, screen_renderer, board_generator};

pub struct GuiData {
    pub cell_size: u32,
    pub offset: i32,
    pub button_width_level_1: u32,
    pub button_width_level_2: u32,
    pub button_height: u32,
    pub y_level_1: i32,
    pub y_level_2: i32,
    pub spacing_level_1: i32,
    pub spacing_level_2: i32,
    pub font_size_buttons: u16,
    pub font_size_numbers: u16,
    pub font_size_message: u16,
    pub font_size_candidates: u16,
}

impl GuiData {
    fn new(window_width: u32, _window_height: u32) -> Self {
        let grid_size = ((window_width as f32) * 0.95) as u32; // f32 used to handle fractional results
        let cell_size = grid_size / 9;
        let offset = ((window_width - grid_size) / 2) as i32;

        let button_width_level_1 = 2 * (cell_size - (offset as u32));
        let button_width_level_2 = 2 * cell_size - 4 * (offset as u32);
        let button_height = cell_size / 8;
        let y_level_1 = ((grid_size + cell_size) as i32) - offset;
        let y_level_2 = ((1.25 * (cell_size as f32)) as i32) + (grid_size as i32) + 2 * offset;

        let number_of_buttons_level_1 = 3;
        let number_of_buttons_level_2 = 5;
        let spacing_level_1 = ((window_width as i32) - 2 * offset) / number_of_buttons_level_1;
        let spacing_level_2 = ((window_width as i32) - 2 * offset) / number_of_buttons_level_2 + 4;

        let font_size_buttons = 20;
        let font_size_numbers = (cell_size / 2) as u16;
        let font_size_message = (cell_size / 2) as u16;
        let font_size_candidates = (cell_size / 4) as u16;

        GuiData {
            cell_size,
            offset,
            button_width_level_1,
            button_width_level_2,
            button_height,
            y_level_1,
            y_level_2,
            spacing_level_1,
            spacing_level_2,
            font_size_buttons,
            font_size_numbers,
            font_size_message,
            font_size_candidates
        }
    }
}

//GameState struct to store all relevant game state information
pub struct GameState {
    pub selected_square: Option<(i32, i32)>,
    pub new_puzzle_button_pressed: bool,
    pub solve_button_pressed: bool,
    pub candidate_button_pressed: bool,
    pub board: Vec<Vec<Option<i32>>>,
    pub initial_board: Vec<Vec<Option<i32>>>,
    pub solved_board: Vec<Vec<Option<i32>>>,
    pub candidates: Vec<Vec<Option<Vec<i32>>>>,
    pub board_initialized: bool,
    pub puzzle_solved: bool,
    pub difficulty: board_generator::BoardDifficulty,
    pub invalid_positions: Vec<(i32, i32, i32)>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            selected_square: None,
            new_puzzle_button_pressed: false,
            solve_button_pressed: false,
            candidate_button_pressed: false,
            board: vec![vec![None; 9]; 9],
            initial_board: vec![vec![None; 9]; 9],
            solved_board: vec![vec![None; 9]; 9],
            candidates: vec![vec![None; 9]; 9],
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

    pub fn is_valid_move(
        &self,
        board: &Vec<Vec<Option<i32>>>,
        row: usize,
        col: usize,
        val: i32
    ) -> bool {
        // Check if the value is already in the row
        if board[row][col] == None {
            return true;
        }

        for i in 0..9 {
            if board[row][i] == Some(val) && i != col {
                print!("found match in row:{}", i);
                return false;
            }
        }
        // Check if the value is already in the column
        for i in 0..9 {
            if board[i][col] == Some(val) && i != row {
                print!("found match in col:{}", i);
                return false;
            }
        }
        // Check if the value is already in the 3x3 square
        let square_row = row - (row % 3);
        let square_col = col - (col % 3);
        for i in square_row..square_row + 3 {
            for j in square_col..square_col + 3 {
                if board[i][j] == Some(val) && i != row && j != col {
                    print!("found match in square:{},{}", i, j);
                    return false;
                }
            }
        }
        return true;
    }
}

pub struct Game {
    pub screen_renderer: ScreenRenderer,
    pub game_state: GameState,
    pub gui_data: GuiData,
}

impl Game {
    pub fn new() -> Game {
        let screen_renderer = ScreenRenderer::new();
        let game_state = GameState::new();
        let (window_width, window_height) = screen_renderer.context.canvas.window().size();
        let gui_data = GuiData::new(window_width, window_height);
        Game {
            screen_renderer,
            game_state,
            gui_data,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let button_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Medium.ttf", self.gui_data.font_size_buttons)?;
        let numbers_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Bold.ttf", self.gui_data.font_size_numbers)?;
        let message_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Bold.ttf", self.gui_data.font_size_message)?;
        let candidates_font = ttf_context.load_font("./assets/fonts/LibreFranklin-Medium.ttf", self.gui_data.font_size_candidates)?;
    
        let mut fonts: Vec<&sdl2::ttf::Font> = Vec::new();
        fonts.push(&button_font);
        fonts.push(&numbers_font);
        fonts.push(&message_font);
        fonts.push(&candidates_font);

        // Game loop
        'running: loop {
            // Handle events
            if !Events::process_events(
                &mut self.game_state,
                &mut self.screen_renderer.context.event_pump,
                &mut self.screen_renderer.context.canvas,
                &self.gui_data
            ) {
                break 'running;
            }

            // Render the screen
            self.screen_renderer.draw(&self.game_state, fonts.clone(), &self.gui_data)?;

            // Set the framerate to 60fps
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
    
}
