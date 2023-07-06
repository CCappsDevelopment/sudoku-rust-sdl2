use sudoku::game::Game;

fn main() -> Result<(), String> {
    let mut game = Game::new();
    game.start()?;
    
    Ok(())
}
