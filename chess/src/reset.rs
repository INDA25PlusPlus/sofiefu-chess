// make board global
use once_cell::sync::Lazy;
use std::sync::Mutex; //guard that only one thread can access it at a time

pub static BOARD: Lazy<Mutex<Vec<Vec<String>>>> = Lazy::new(|| {
    Mutex::new(vec![vec!["empty".to_string(); 9]; 9]) 
});
pub static COLOR: Lazy<Mutex<Vec<Vec<char>>>> = Lazy::new(|| { 
    Mutex::new(vec![vec![' '; 9]; 9]) 
});
pub static TURN: Lazy<Mutex<char>> = Lazy::new(|| Mutex::new('w'));


pub fn reset_board() {
    //lock for mutex guard to r/w safely. 
    // unwrap handles possibly posioned lock
    let mut board = BOARD.lock().unwrap(); 
    let mut color = COLOR.lock().unwrap();
    let mut turn = TURN.lock().unwrap();

    // reset board
    turn = 'w';
    for i in 1..9 { 
        for u in 1..9{
            board[i][u] = "empty".to_string();
            color[i][u] = ' '; 
        }
    }

    let alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    // Pawns
    for letter in alphabet {
        let x = letter as usize - 96;

        board[2][x] = "pawn".to_string();
        color[2][x] = 'w';   

        board[7][x] = "pawn".to_string();
        color[7][x] = 'b';
    }

    // Other pieces
    let rows = vec![8, 1];
    for &row in &rows {
        let c = if row==8 { 'b' } else { 'w' }; 

        board[row][1] = "rook".to_string(); color[row][0] = c;
        board[row][8] = "rook".to_string(); color[row][7] = c;

        board[row][2] = "knight".to_string(); color[row][1] = c;
        board[row][7] = "knight".to_string(); color[row][6] = c;

        board[row][3] = "bishop".to_string(); color[row][2] = c;
        board[row][6] = "bishop".to_string(); color[row][5] = c;

        board[row][4] = "queen".to_string(); color[row][3] = c;
        board[row][5] = "king".to_string(); color[row][4] = c;
    }

    // Example print
    // println!("{}", board[2][2].as_ref().unwrap());
}
