use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;

use crate::board_generator;
use crate::game::{ GameState, GuiData };
use crate::sdl_context::SdlContext;

pub struct ScreenRenderer {
    pub context: SdlContext,
    pub window_width: i32,
    pub window_height: i32,
}

impl ScreenRenderer {
    pub fn new() -> ScreenRenderer {
        let context = SdlContext::new();
        let window_width = context.canvas.viewport().width() as i32;
        let window_height = context.canvas.viewport().height() as i32;

        ScreenRenderer {
            context,
            window_width,
            window_height,
        }
    }

    // Render the screen
    pub fn draw(
        &mut self,
        game_state: &GameState,
        fonts: Vec<&sdl2::ttf::Font>,
        gui_data: &GuiData
    ) -> Result<(), String> {
        self.update();

        self.draw_bg();
        self.draw_board(game_state, &gui_data)?;

        // Draw the numbers
        if game_state.board_initialized {
            self.draw_invalid_positions(&game_state, &gui_data)?;
            self.draw_numbers(&game_state, &fonts, &gui_data)?;
        }
        else if !game_state.board_initialized && game_state.puzzle_solved {
            self.display_gameover_message(&fonts[2], &gui_data)?;
        }
        
        self.draw_buttons(&game_state, fonts, gui_data)?;

        self.context.canvas.present();

        Ok(())
    }

    fn draw_bg(&mut self) {
        // Set the background color and clear the screen
        self.context.canvas.set_draw_color(Color::RGB(245, 242, 232));
        self.context.canvas.clear();
    }

    fn draw_board(
        &mut self,
        game_state: &GameState,
        gui_data: &GuiData
    ) -> Result<(), String> {
        // Draw the board and the numbers
        self.context.canvas.set_draw_color(Color::RGB(26, 28, 26));
        for i in 0..9 {
            for j in 0..9 {
                let x = (j as i32) * (gui_data.cell_size as i32) + gui_data.offset;
                let y = (i as i32) * (gui_data.cell_size as i32) + gui_data.offset;
                let rect = Rect::new(x, y, gui_data.cell_size, gui_data.cell_size);
                if Some((i, j)) == game_state.selected_square {
                    self.context.canvas.draw_rect(rect)?;
                    let selected_rect = Rect::new(
                        x + (1 as i32),
                        y + (1 as i32),
                        gui_data.cell_size - 2,
                        gui_data.cell_size - 2
                    );
                    self.context.canvas.set_draw_color(Color::RGB(243, 206, 161));
                    self.context.canvas.fill_rect(selected_rect)?;
                } else {
                    if game_state.initial_board[i as usize][j as usize].is_some() {
                        self.context.canvas.set_draw_color(Color::RGB(225, 223, 216));
                        self.context.canvas.fill_rect(rect)?;
                    }
                    self.context.canvas.set_draw_color(Color::RGB(26, 28, 26));
                    self.context.canvas.draw_rect(rect)?;
                }
            }
        }

        // Drawing thicker lines for the Sudoku's 3x3 grid
        for i in 0..4 {
            let x = i * gui_data.cell_size * 3 + (gui_data.offset as u32);
            let y = i * gui_data.cell_size * 3 + (gui_data.offset as u32);
            let line = Rect::new(x as i32, gui_data.offset, 3, gui_data.cell_size * 9);
            let column = Rect::new(gui_data.offset, y as i32, gui_data.cell_size * 9, 3);
            self.context.canvas.set_draw_color(Color::RGB(26, 28, 26));
            self.context.canvas.fill_rect(line)?;
            self.context.canvas.fill_rect(column)?;
        }

        Ok(())
    }

    fn draw_invalid_positions(&mut self, game_state: &GameState, gui_data: &GuiData) -> Result<(), String> {
        // Draw invalid positions
        for (x, y, _val) in &game_state.invalid_positions {
            let rect_x = y * (gui_data.cell_size as i32) + gui_data.offset;
            let rect_y = x * (gui_data.cell_size as i32) + gui_data.offset;
            let invalid_rect = Rect::new(
                rect_x + 1,
                rect_y + 1,
                gui_data.cell_size - 2,
                gui_data.cell_size - 2
            );
            let board_rect = Rect::new(
                rect_x + 4,
                rect_y + 4,
                gui_data.cell_size - 9,
                gui_data.cell_size - 9
            );
            self.context.canvas.set_draw_color(Color::RGB(190, 0, 0));
            self.context.canvas.fill_rect(invalid_rect)?;
            self.context.canvas.set_draw_color(Color::RGB(245, 242, 232));
            self.context.canvas.fill_rect(board_rect)?;
        }

        Ok(())
    }

    fn draw_numbers(
        &mut self,
        game_state: &GameState,
        fonts: &Vec<&sdl2::ttf::Font>,
        gui_data: &GuiData
    ) -> Result<(), String> {
        // Draw the numbers
        for i in 0..9 {
            for j in 0..9 {
                let x = j * (gui_data.cell_size as i32) + gui_data.offset;
                let y = i * (gui_data.cell_size as i32) + gui_data.offset;

                let texture_creator = self.context.canvas.texture_creator();

                if let Some(candidates) = &game_state.candidates[i as usize][j as usize] {
                    if !candidates.is_empty() {
                        let mut sorted_candidates = candidates.clone();
                        sorted_candidates.sort();

                        for (idx, &val) in sorted_candidates.iter().enumerate() {
                            let x_offset = (idx % 3) * ((gui_data.cell_size as usize) / 3);
                            let y_offset = (idx / 3) * ((gui_data.cell_size as usize) / 3);

                            let surface = fonts[3]
                                .render(&val.to_string())
                                .blended(Color::RGB(0, 0, 0))
                                .map_err(|e| e.to_string())?;

                            let texture = texture_creator
                                .create_texture_from_surface(&surface)
                                .map_err(|e| e.to_string())?;

                            let TextureQuery { width, height, .. } = texture.query();

                            let target = Rect::new(
                                (x as i32) +
                                    (x_offset as i32) +
                                    ((gui_data.cell_size as i32) / 3 - (width as i32)) / 2,
                                (y as i32) +
                                    (y_offset as i32) +
                                    ((gui_data.cell_size as i32) / 3 - (height as i32)) / 2,
                                width,
                                height
                            );

                            self.context.canvas.copy(&texture, None, Some(target))?;
                        }
                        continue;
                    }
                }

                if let Some(val) = game_state.board[i as usize][j as usize] {
                    let surface = fonts[1]
                        .render(&val.to_string())
                        .blended(Color::RGB(0, 0, 0))
                        .map_err(|e| e.to_string())?;

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .map_err(|e| e.to_string())?;

                    let TextureQuery { width, height, .. } = texture.query();

                    let target = Rect::new(
                        (x as i32) + ((gui_data.cell_size as i32) - (width as i32)) / 2,
                        (y as i32) + ((gui_data.cell_size as i32) - (height as i32)) / 2,
                        width,
                        height
                    );

                    self.context.canvas.copy(&texture, None, Some(target))?;
                }
            }
        }

        Ok(())
    }

    fn draw_buttons(
        &mut self,
        game_state: &GameState,
        fonts: Vec<&sdl2::ttf::Font>,
        gui_data: &GuiData
    ) -> Result<(), String> {
        let button_names_level_1 = vec!["New Puzzle", "Candidate", "Solve"];
        let button_states_level_1 = vec![
            Some(game_state.new_puzzle_button_pressed),
            Some(game_state.candidate_button_pressed),
            Some(game_state.solve_button_pressed)
        ];

        let button_names_level_2 = vec!["Beginner", "Easy", "Medium", "Hard", "Expert"];
        let button_difficulties_level_2 = vec![
            Some(board_generator::BoardDifficulty::Beginner),
            Some(board_generator::BoardDifficulty::Easy),
            Some(board_generator::BoardDifficulty::Medium),
            Some(board_generator::BoardDifficulty::Hard),
            Some(board_generator::BoardDifficulty::Expert)
        ];

        for (index, (button_name, button_state)) in button_names_level_1
            .iter()
            .zip(button_states_level_1)
            .enumerate() {
            let x = gui_data.spacing_level_1 * (index as i32) + 4 * gui_data.offset;
            self.draw_button(
                game_state,
                &fonts[0],
                x,
                gui_data.y_level_1,
                gui_data.button_width_level_1,
                gui_data.button_height,
                button_name,
                button_state,
                None
            )?;
        }

        for (index, (button_name, button_difficulty)) in button_names_level_2
            .iter()
            .zip(button_difficulties_level_2)
            .enumerate() {
            let x = gui_data.spacing_level_2 * (index as i32) + 2 * gui_data.offset;
            self.draw_button(
                game_state,
                &fonts[0],
                x,
                gui_data.y_level_2,
                gui_data.button_width_level_2,
                gui_data.button_height,
                button_name,
                None,
                button_difficulty
            )?;
        }

        Ok(())
    }

    fn draw_button(
        &mut self,
        game_state: &GameState,
        font: &sdl2::ttf::Font,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        text: &str,
        button_pressed: Option<bool>,
        difficulty: Option<board_generator::BoardDifficulty>
    ) -> Result<(), String> {
        // Set the button color based on its pressed state
        let button_color = if
            button_pressed == Some(true) ||
            difficulty == Some(game_state.difficulty)
        {
            Color::RGB(243, 206, 161)
        } else {
            Color::RGB(245, 242, 232)
        };

        let border_color = Color::RGB(26, 28, 26);

        let x_offset: u32 = 30;
        let y_offset: u32 = 30;

        let border_rect = Rect::new(
            x - ((x_offset as i32) + 10) / 2,
            y - 25,
            width + x_offset + 15,
            height + y_offset + 15
        );
        let button_rect = Rect::new(
            x - (x_offset as i32) / 2,
            y - 20,
            width + x_offset,
            height + y_offset
        );

        // Draw the button border
        self.context.canvas.set_draw_color(border_color);
        self.context.canvas.fill_rect(border_rect)?;
        // Fill the button with the button color
        self.context.canvas.set_draw_color(button_color);
        self.context.canvas.fill_rect(button_rect)?;

        // Render and draw the button text
        let surface = font
            .render(text)
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;

        let texture_creator = self.context.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width: texture_width, height: texture_height, .. } = texture.query();

        let target = Rect::new(
            button_rect.x() + ((button_rect.width() as i32) - (texture_width as i32)) / 2,
            button_rect.y() + ((button_rect.height() as i32) - (texture_height as i32)) / 2,
            texture_width,
            texture_height
        );
        self.context.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    fn display_gameover_message(
        &mut self,
        font: &sdl2::ttf::Font,
        gui_data: &GuiData
    ) -> Result<(), String> {
        // Render and draw the game over message
        let surface = font
            .render("You solved the puzzle!")
            .blended(Color::RGB(26, 28, 26))
            .map_err(|e| e.to_string())?;

        let texture_creator = self.context.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let TextureQuery { width: texture_width, height: texture_height, .. } = texture.query();

        let x = gui_data.offset + (((gui_data.cell_size * 9) as i32) - (texture_width as i32)) / 2;
        let y = gui_data.offset + (((gui_data.cell_size * 9) as i32) - (texture_height as i32)) / 2;
        let x_offset: u32 = 30;
        let y_offset: u32 = 30;
        let target = Rect::new(x, y, texture_width, texture_height);
        let bg_rect = Rect::new(
            x - (x_offset as i32) / 2,
            y - 20,
            texture_width + x_offset,
            texture_height + y_offset
        );
        let border_rect = Rect::new(
            x - ((x_offset as i32) + 10) / 2,
            y - 25,
            texture_width + x_offset + 20,
            texture_height + y_offset + 20
        );

        self.context.canvas.set_draw_color(Color::RGB(26, 28, 26));
        self.context.canvas.fill_rect(border_rect)?;
        self.context.canvas.set_draw_color(Color::RGB(245, 242, 232));
        self.context.canvas.fill_rect(bg_rect)?;

        self.context.canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }


    // TODO: seperate out update logic
    pub fn update(&mut self) {}
}
