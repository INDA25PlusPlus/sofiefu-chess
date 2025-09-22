pub mod movement; // mod declares move as a submodule of game
pub mod special_moves;
pub mod generate_valid_moves;
pub mod check;
#[derive(Clone)]

pub struct game_state {
    pub board: Vec<Vec<String>>,
    pub player: Vec<Vec<char>>,
    pub turn: char,
    pub castle: Vec<bool>,
}

impl game_state {
    pub fn new() -> Self { // creates a new instance of my struct
        let mut board = vec![vec!["empty".to_string(); 9]; 9];
        let mut player = vec![vec![' '; 9]; 9];

        let alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        // Pawns
        for letter in alphabet {
            let x = letter as usize - 96;

            board[2][x] = "pawn".to_string();
            player[2][x] = 'w';   

            board[7][x] = "pawn".to_string();
            player[7][x] = 'b';
        }

        // Other pieces
        let rows = vec![8, 1];
        for &row in &rows {
            let c = if row==8 { 'b' } else { 'w' }; 

            board[row][1] = "rook".to_string(); player[row][1] = c;
            board[row][8] = "rook".to_string(); player[row][8] = c;

            board[row][2] = "knight".to_string(); player[row][2] = c;
            board[row][7] = "knight".to_string(); player[row][7] = c;

            board[row][3] = "bishop".to_string(); player[row][3] = c;
            board[row][6] = "bishop".to_string(); player[row][6] = c;

            board[row][4] = "queen".to_string(); player[row][4] = c;
            board[row][5] = "king".to_string(); player[row][5] = c;
        }

        // Example print
        // println!("{}", board[2][2].as_ref().unwrap());

        Self { // returns the an instance of my struct
            board, 
            player, 
            turn: 'w',
            castle: vec![true, true]
        }
    }

    pub fn switch_turn(&mut self){
        self.turn = if self.turn=='w' {'b'} else {'w'};
    }

    pub fn reset(&mut self, r: i32, c: i32){
        let ur = r as usize; let uc = c as usize;
        self.board[ur][uc] = "empty".to_string();
        self.player[ur][uc] = ' ';
    }

    pub fn set(&mut self, r: i32, c: i32, new_piece: String, new_player: char){
        let ur = r as usize; let uc = c as usize;
        self.board[ur][uc] = new_piece;
        self.player[ur][uc] = new_player;
    }

    pub fn move_piece(&mut self, sr: i32, sc: i32, er: i32, ec: i32) { // move (sr, sc)->(er, ec)
        let usr = sr as usize; let usc = sc as usize;
        let uer = er as usize; let uec = ec as usize;
        self.board[uer][uec] = self.board[usr][usc].clone(); self.player[uer][uec] = self.player[usr][usc];
        self.board[usr][usc] = "empty".to_string(); self.player[usr][usc] = ' ';

    }

    pub fn get_piece(&self, r: i32, c: i32) -> String {
        let ur = r as usize; let uc = c as usize;
        return self.board[ur][uc].clone();
    }

    pub fn get_player(&self, r: i32, c: i32) -> char {
        let ur = r as usize; let uc = c as usize;
        return self.player[ur][uc];
    }

    pub fn empty(&self, r: i32, c: i32) -> bool {
        let ur = r as usize; let uc = c as usize;
        return self.board[ur][uc] == "empty".to_string();
    }
}
