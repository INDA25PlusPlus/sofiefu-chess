use crate::game::game_state; 
use crate::outcome::Outcome;

impl game_state {
    pub fn castle(&mut self) -> Outcome { // castle[0]=white, castle[1]=black
        if self.turn == 'w' && self.castle[0] && self.empty(1, 6) && self.empty(1, 7) {
            self.set(1, 7, "king".to_string(), self.turn); self.reset(1, 5);
            self.set(1, 6, "rook".to_string(), self.turn); self.reset(1, 8);
            self.castle[0] = false;
            self.switch_turn();
            return Outcome::Valid;
        }
        else if self.turn == 'b' && self.castle[1] && self.empty(8, 2) && self.empty(8, 3) {
            self.set(8, 2, "king".to_string(), self.turn); self.reset(8, 4);
            self.set(8, 3, "rook".to_string(), self.turn); self.reset(8, 1);
            self.castle[1] = false;
            self.switch_turn();
            return Outcome::Valid;
        }
        return Outcome::Invalid;
    
        // CHECK FOR CHECKING - unfinished 

    }
    
    pub fn promotion(&mut self, start_r: i32, start_c: i32, end_r: i32, end_c:i32, new_piece: String) -> Outcome {
        // some checks
        let choices = vec!["knight", "bishop", "rook", "queen", "king"]; // for promotion
        if !self.in_range(start_r, start_c) || !self.in_range(end_r, end_c) {return Outcome::Bad_coordinates;}
        let start_piece = self.get_piece(start_r, start_c); let start_player = self.get_player(start_r, start_c);
    
        if start_piece == "empty".to_string() { return Outcome::Bad_coordinates; }
        else if start_player != self.turn { return Outcome::Invalid; }
        if !choices.contains(&new_piece.as_str()) {return Outcome::Invalid; }
    
        // generate valid moves + check if end position exists
        let valid_moves = self.pawn_valid_moves(start_r, start_c);
        if !valid_moves.contains(&(end_r, end_c)) { return Outcome::Invalid; }
    
        // CHECKED?
        let opponent = if start_player=='w' {'b'} else {'w'};
        let mut upd_game = self.clone();   
        upd_game.set(end_r, end_c, new_piece, start_player);
        upd_game.reset(start_r, start_c);
        if upd_game.is_checking(opponent) { return Outcome::Checked; }

        // SUCCESSFUL MOVE
        self.move_piece(start_r, start_c, end_r, end_c);
        self.switch_turn();
    
        // CHECK(MATE)?
        if self.is_checking(start_player) {
            if self.is_checkmate() { return Outcome::Checkmate; } // game should end here
            return Outcome::Check; 
        }
        return Outcome::Valid;
    }
}