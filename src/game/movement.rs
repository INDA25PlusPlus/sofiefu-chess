//crate is a path prefix, "start from top level of this library and navigate to this module"
use crate::game::game_state; 
use crate::outcome::Outcome;

impl game_state {
    pub fn make_move(&mut self, start_r: i32, start_c: i32, end_r: i32, end_c: i32) -> Outcome { 
        // SOME CHECKS + ASSIGNMENTS
        if !self.in_range(start_r, start_c) || !self.in_range(end_r, end_c) { return Outcome::Bad_coordinates; }
    
        let start_piece = self.get_piece(start_r, start_c); let end_piece = self.get_piece(end_r, end_c);
        let start_player = self.get_player(start_r, start_c); let end_player = self.get_player(end_r, end_c);
    
        if start_piece == "empty".to_string() { return Outcome::Bad_coordinates; }
        else if start_player != self.turn { return Outcome::Wrong_player; }
    
        // GENERATE VALID POSITIONS
        let valid_pos = self.valid_moves(start_r, start_c); 
        if !valid_pos.contains(&(end_r, end_c)) { return Outcome::Invalid; }
    
        
        // CHECKED?
        let opponent = if start_player=='w' {'b'} else {'w'};
        let mut upd_game = self.clone();   
        upd_game.move_piece(start_r, start_c, end_r, end_c);
        if upd_game.is_checking(start_player) { return Outcome::Checked; }
        
        // SUCCESSFUL MOVE
        self.move_piece(start_r, start_c, end_r, end_c);
        self.switch_turn();
    
        // update castling opportunity
        if ((start_r, start_c)==(1, 8)) || (start_r, start_c)==(1, 5) { self.castle[0] = false;}
        if ((start_r, start_c)==(8, 1)) || (start_r, start_c)==(8, 4) { self.castle[0] = false; }
    
        // CHECK(MATE)?
        if self.is_checking(start_player) {
            if self.is_checkmate() { return Outcome::Checkmate; } // game should end here
            return Outcome::Check;
        }
        return Outcome::Valid;
    }
}
