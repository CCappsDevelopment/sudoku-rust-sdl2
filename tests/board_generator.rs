use sudoku::board_generator;

#[test]
fn test_get_square() {
    let difficulty = board_generator::BoardDifficulty::Easy;
    let board_generator = board_generator::BoardGenerator::new(difficulty);
    let mut board: Vec<Vec<Option<i32>>> = vec![vec![Some(1); 9]; 9];
    board[0][0] = Some(9);
    board[1][1] = Some(9);
    board[2][2] = Some(9);
    
    assert_eq!(board_generator.get_square(&board, 0, 0), 
        vec![
            vec![Some(9), Some(1), Some(1)], 
            vec![Some(1), Some(9), Some(1)], 
            vec![Some(1), Some(1), Some(9)]
        ]);
}

#[test]
fn test_is_valid_false() {
    let difficulty = board_generator::BoardDifficulty::Easy;
    let board_generator = board_generator::BoardGenerator::new(difficulty);
    let board = vec![
        vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)], 
        vec![Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)], 
        vec![Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)], 
        vec![Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)], 
        vec![Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4)], 
        vec![Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)], 
        vec![Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)], 
        vec![Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5)], 
        vec![Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]
    ];

    assert_eq!(board_generator.is_valid(&board, 0, 0, 9), false);
}

#[test]
fn test_is_valid_true() {
    let difficulty = board_generator::BoardDifficulty::Easy;
    let board_generator = board_generator::BoardGenerator::new(difficulty);
    let board = vec![
        vec![None, Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9)], 
        vec![Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3)], 
        vec![Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)], 
        vec![Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1)], 
        vec![Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4)], 
        vec![Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)], 
        vec![Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(1), Some(2)], 
        vec![Some(6), Some(7), Some(8), Some(9), Some(1), Some(2), Some(3), Some(4), Some(5)], 
        vec![Some(9), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]
    ];

    assert_eq!(board_generator.is_valid(&board, 0, 0, 1), true);
}