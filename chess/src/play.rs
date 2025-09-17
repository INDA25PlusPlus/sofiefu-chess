//crate is a path prefix, "start from top level of this library and navigate to this module"
use crate::reset::game_state; 

fn in_range(r: i32, c: i32) -> bool {
    if r<1 || r>8 || c<1 || c>8 {return false;}
    return true;
}

fn valid_moves(r: i32, c: i32, game: &mut game_state) -> Vec<(i32, i32)>{
    let mut valid_pos: Vec<(i32, i32)> = Vec::new();
    let mut add: Vec<(i32, i32)> = Vec::new();
    let mut range = 0;

    let piece = game.get_piece(r, c); let player = game.get_player(r, c);

    // note what color
    if piece == "pawn".to_string() {
        let sign = if player=='w' {1} else {-1};
        add.push((sign, 0));
        if ((r==2 && player=='w') || (r==7 && player=='b')) { add.push((2*sign, 0)); }
        range = 1;
        // en passant
    }
    else if piece == "rook".to_string() {
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
        range = 8;
    }
    else if piece == "bishop".to_string() {
        add = vec![(1, 1), (1, -1), (-1, -1), (-1, 1)]; 
        range = 8;
    }
    else if piece == "knight".to_string() {
        add = vec![(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)];
        range = 1;
    }
    else if piece == "queen".to_string() {
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        range = 8;
    }
    else if piece == "king".to_string() { 
        add = vec![(1, 0), (-1, 0), (0, -1), (0, 1), (1, 1), (1, -1), (-1, -1), (-1, 1)];
        range = 1;
    }

    for &(addr, addc) in &add { // iterate by reference
        for ustep in 1..(range+1) {
            let step = ustep as i32;
            let new_r = r+step*addr; let new_c = c+step*addc;
            if !in_range(new_r, new_c) {break;}

            if game.get_piece(new_r, new_c) != "empty".to_string() {
                if player != game.get_player(new_r, new_c) { valid_pos.push((new_r, new_c)); }
                break;
            }
            valid_pos.push((new_r, new_c));
        }
    }

    return valid_pos;
}

// fn checkmate 

fn dangerous_pos(r: i32, c: i32, game: &mut game_state) -> bool{
    for i in 1..9 {
        for u in 1..9 {
            let ui = i as i32; let uu = u as i32;
            if game.board[i][u] != "empty".to_string() && game.get_player(r, c) != game.player[i][u] {
                let mut valid_pos = valid_moves(ui, uu, game);
                if valid_pos.contains(&(r, c)) { return true; }
            }
        }
    }
    
    return false;
}

fn is_checked(game: &mut game_state) -> bool{
    // find position of my king
    let mut r: i32 = 1; let mut c: i32 = 1;
    for i in 1..9 {
        for u in 1..9 {
            if game.board[i][u] == "king".to_string() && game.player[i][u] == game.turn {
                r = i as i32; c = u as i32;
            }
        }
    }
    return dangerous_pos(r, c, game);
}


fn is_checkmate(game: &mut game_state) -> bool{
    let mut r: i32 = 1; let mut c: i32 = 1;
    for i in 1..9 {
        for u in 1..9 {
            if game.board[i][u] != "empty".to_string() {    
                let ui = i as i32; let uu = u as i32;
                let mut valid_pos = valid_moves(ui, uu, game);
                for &(a, b) in &valid_pos {
                    if game.get_piece(a, b) == "king".to_string() && game.get_player(a, b)!=game.turn {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

// 0=invalid move, 1=valid move, 2=valid move piece eaten
pub fn make_move(start_r: i32, start_c: i32, end_r: i32, end_c: i32, game: &mut game_state) -> String { 
    // check if start and ending position is valid + piece exists
    if !in_range(start_r, start_c) || !in_range(end_r, end_c) {return "invalid move".to_string();}

    // for cleaner code
    let start_piece = game.get_piece(start_r, start_c); let end_piece = game.get_piece(end_r, end_c);
    let start_player = game.get_player(start_r, start_c); let end_player = game.get_player(end_r, end_c);

    // some more checks
    if start_piece == "empty".to_string() { return "invalid: no piece".to_string(); }
    else if start_player != game.turn { return "invalid: not your turn".to_string(); }
    else if start_player == end_player { return "invalid: eating up own piece".to_string(); }

    // generate valid positions + check if end is there
    let mut valid_pos = Vec::new();
    valid_pos = valid_moves(start_r, start_c, game); 
    if !valid_pos.contains(&(end_r, end_c)) { return "invalid move".to_string(); }
    
    // check that the current player is not being checkmated
    // check if any checkmate --> must be on me
    // not checked

    // dont move into checked position 
    if start_piece == "king".to_string() {
        if dangerous_pos(end_r, end_c, game) {return "invalid: moving into checkmate".to_string(); }
    }

    // check promotion
    
    // check if current player is checked
    game.set(end_r, end_c, start_piece.clone(), start_player); 
    game.reset(start_r, start_c);

    
    if is_checked(game) { 
        game.set(start_r, start_c, start_piece.clone(), start_player);
        game.set(end_r, end_c, end_piece.clone(), end_player);
        return "invalid move: you are being checked".to_string(); 
    }

    // check if checkmating opponent
    if is_checkmate(game) {
        return "checkmate".to_string();
    }
    
    // succesful move + switch turn
    game.switch_turn();
    if end_piece == "empty".to_string() { 
        return "successful move: no piece eaten".to_string(); 
    }
    else{
        return "piece eaten".to_string(); // return which piece
    }
}

// possible optimization: take a1 as input instead of 4 numbers
