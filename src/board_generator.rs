use rand::{seq::SliceRandom, Rng};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BoardDifficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Expert,
}

pub struct BoardGenerator {
    difficulty: BoardDifficulty,
    solved_board: Option<Vec<Vec<Option<i32>>>>,
}

impl BoardGenerator {
    pub fn new(difficulty: BoardDifficulty) -> BoardGenerator {
        BoardGenerator { difficulty, solved_board: None }
    }

    pub fn get_solved_board(&self) -> Option<Vec<Vec<Option<i32>>>> {
        self.solved_board.clone()
    }

    pub fn generate_sudoku(&mut self) -> Vec<Vec<Option<i32>>> {
        let mut rng = rand::thread_rng();
        
        loop {
            let mut board_raw: Vec<Vec<Option<i32>>> = vec![vec![None; 9]; 9];
            if self.fill_board(&mut board_raw, 0) {
                self.solved_board = Some(board_raw.clone());
    
                let removals = match self.difficulty {
                    BoardDifficulty::Beginner => 30,
                    BoardDifficulty::Easy => 45,
                    BoardDifficulty::Medium => 55,
                    BoardDifficulty::Hard => 62,
                    BoardDifficulty::Expert => 64,
                };
    
                for _ in 0..removals {
                    loop {
                        let row = rng.gen_range(0..9);
                        let col = rng.gen_range(0..9);
                        if !board_raw[row][col].is_none() {
                            board_raw[row][col] = None;
                            break;
                        }
                    }
                }
                return board_raw;
            }
        }
    }
    
    pub fn _format_board(&self, board: &Vec<Vec<Option<i32>>>) -> String {
        board.iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.map_or("0".to_string(), |value| value.to_string()))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn fill_board(&self, board: &mut Vec<Vec<Option<i32>>>, position: usize) -> bool {
        if position == 81 {
            return true;
        }
    
        let row = position / 9;
        let col = position % 9;
    
        if !board[row][col].is_none() {
            return self.fill_board(board, position + 1);
        }
    
        let mut rng = rand::thread_rng();
        let mut numbers: Vec<i32> = (1..=9).collect();
        numbers.shuffle(&mut rng);
    
        for num in numbers {
            if self.is_valid(board, row, col, num) {
                board[row][col] = Some(num);
                if self.fill_board(board, position + 1) {
                    return true;
                }
                board[row][col] = None;
            }
        }
    
        false
    }

    pub fn get_square(&self, board: &Vec<Vec<Option<i32>>>, row: usize, col: usize) -> Vec<Vec<Option<i32>>> {
        let start_row = row - (row % 3);
        let start_col = col - (col % 3);
        let mut result = Vec::new();

        for i in start_row..(start_row + 3) {
            let mut row_slice = Vec::new();
            for j in start_col..(start_col + 3) {
                row_slice.push(board[i][j]);
            }
            result.push(row_slice);
        }

        result
    }

    pub fn is_valid(&self, board: &Vec<Vec<Option<i32>>>, row: usize, col: usize, num: i32) -> bool {
        !(// check if the number already exists in the row
          (0..9).any(|i| board[row][i].unwrap_or_default() == num)
          // check if the number already exists in the column
          || (0..9).any(|i| board[i][col].unwrap_or_default() == num)
          // check if the number already exists in the 3x3 square
          || self.get_square(board, row, col)
                .iter()
                .flatten()
                .any(|&x| x == Some(num)))
    }
    
}