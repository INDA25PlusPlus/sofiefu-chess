//crate is a path prefix, "start from top level of this library and navigate to this module"
use crate::reset::{BOARD, COLOR}; 

fn in_range(r: i32, c: i32) -> bool {
    if r<1 || r>8 || c<1 || c>8 {return false;}
    return true;
}

fn valid_moves(r: i32, c: i32, my_color: char, board: &mut Vec<Vec<String>>) -> Vec<(i32, i32)>{
    let ur = r as usize; let uc = c as usize;
    let mut valid_pos: Vec<(i32, i32)> = Vec::new();
    let mut add: Vec<(i32, i32)> = Vec::new();
    let mut range = 0;
    let piece_type = board[ur][uc].clone();

    // note what color
    if piece_type == "pawn".to_string() {
        let sign = if my_color=='w' {1} else {-1};
        add.push((sign, 0));
        if ((r==2 && my_color=='w') || (r==7 && my_color=='b')) { add.push((2*sign, 0)); }
        range = 1;
        // en passant
    }
    else if piece_type == "rook".to_string() {
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
        range = 8;
    }
    else if piece_type == "bishop".to_string() {
        add = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)]; 
        range = 8;
    }
    else if piece_type == "knight".to_string() {
        add = vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
        range = 1;
    }
    else if piece_type == "queen".to_string() {
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        range = 8;
    }
    else if piece_type == "king".to_string() { 
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        range = 1;
    }

    for &(addr, addc) in &add { // iterate by reference
        for ustep in 1..(range+1) {
            let step = ustep as i32;
            let new_r = r+step*addr; let new_c = c+step*addc;
            if !in_range(new_r, new_c) {break;}

            valid_pos.push((new_r, new_c));
            let unew_r = new_r as usize; let unew_c = new_c as usize;
            if board[unew_r][unew_c] != "empty".to_string() {break;}
        }
    }

    return valid_pos;
}

// fn checkmate 

fn dangerous_pos(r: i32, c: i32, color: &mut Vec<Vec<char>>, board: &mut Vec<Vec<String>>) -> bool{
    for i in 1..9 {
        for u 1..9 {
            let ui = i as i32; let uu = u as i32;
            if board[i][u] != "empty".to_string() {
                let mut valid_pos = valid_moves(ui, uu, color[i][u], board);
                if !valid_pos.contains(&(r, c)) { return true; }
            }
        }
    }
    return false;
}

fn is_checked(player: char, color: &mut Vec<Vec<char>>, board: &mut Vec<Vec<String>>) -> bool{
    let mut r: i32 = 1; 
    let mut c: i32 = 1;
    for i in 1..9 {
        for u 1..9 {
            if board[i][u] != "king".to_string() && color[i][u] == player {
                r = i as i32; c = u as i32;
            }
        }
    }
    for i in 1..9 {
        for u 1..9 {
            let ui = i as i32; let uu = u as i32;
            if board[i][u] != "empty".to_string() && color[i][u] != player {
                let mut valid_pos = valid_moves(ui, uu, color[i][u], board);
                if !valid_pos.contains(&(r, c)) { return true; }
            }
        }
    }
    return false;
}

// 0=invalid move, 1=valid move, 2=valid move piece eaten
pub fn make_move(start_r: i32, start_c: i32, end_r: i32, end_c: i32) -> String { 
    let mut board = BOARD.lock().unwrap();
    let mut color = COLOR.lock().unwrap();
    let mut turn = TURN.lock().unwrap();

    // check if start and ending position is valid + piece exists
    if !in_range(start_r, start_c) || !in_range(end_r, end_c) {return "invalid move".to_string();}

    // check if piece exists
    let ustart_r = start_r as usize; let ustart_c = start_c as usize;
    let piece_type = board[ustart_r][ustart_c].clone().as_str();
    let my_color = color[ustart_r][ustart_c];
    if piece_type == "empty" { return "invalid move".to_string(); }
    else if turn != my_color { return "not your turn".to_string(); }
    

    // generate valid positions + check if end is there
    let mut valid_pos = Vec::new();
    valid_pos = valid_moves(start_r, start_c, color[ustart_r][ustart_c], &mut board); 
    if !valid_pos.contains(&(end_r, end_c)) { return "invalid move".to_string(); }
    
    // check that the current player is not being checkmated
    // check if any checkmate --> must be on me
    // not checked

    // dont move into checked position 
    if piece_type == "king" {
        if dangerous_pos(end_r, end_c, &mut color, &mut board) {return "moving into checkmate".to_string(); }
    }

    
    // check promotion
    // let uend_r = end_r as usize; let uend_c = end_c as usize;
    // if board[ustart_r][ustart_c] == ""
    
    
    // check if current player is checked
    let prev_piece = board[uend_r][uend_c].clone(); // eaten piece
    let prev_color = color[uendr][uend_c];
    board[uend_r][uend_c] = board[ustart_r][ustart_c].clone(); board[ustart_r][ustart_c] = "empty".to_string();
    color[uend_r][uend_c] = color[ustart_r][ustart_c]; color[ustart_r][ustart_c] = ' ';
    
    if is_checked(turn, &mut color, &mut board) { 
        board[ustart_r][ustart_c] = board[uend_r][uend_c].clone();
        board[uend_r][uend_c] = prev_piece; 
        color[ustart_r][ustart_c] = color[uend_r][uend_c];
        color[uend_r][uend_c] = prev_color; 
        return "invalid move, resolve check".to_string(); 
    }
    
    // succesful move + switch turn
    turn = if turn {false;} else {true;};
    if prev_piece == "empty".to_string() { 
        return "successful move: no piece eaten".to_string(); 
    }
    else{
        return "piece eaten".to_string(); // return which piece
    }
}

// possible optimization: take a1 as input instead of 4 numbers