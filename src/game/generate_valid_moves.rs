use crate::game::game_state; 

impl game_state {
    pub fn in_range(&mut self, r: i32, c: i32) -> bool {
        if r<1 || r>8 || c<1 || c>8 {return false;}
        return true;
    }
    
    pub fn pawn_valid_moves(&mut self, r: i32, c: i32) -> Vec<(i32, i32)> {
        let player = self.get_player(r, c);
        let mut valid_pos: Vec<(i32, i32)> = Vec::new();
        let sign = if player=='w' {1} else {-1};
        if self.in_range(r+sign, c) && self.empty(r+sign, c) { 
            valid_pos.push((r+sign, c)); 
            if ((r==2 && player=='w') || (r==7 && player=='b')) && self.empty(r+2*sign, c) { valid_pos.push((r+2*sign, c)); }
        }
        if self.in_range(r+sign, c+1) && !self.empty(r+sign, c+1) && self.get_player(r, c)!=self.get_player(r+sign, c+1) { valid_pos.push((r+sign, c+1)); }
        if self.in_range(r+sign, c-1) && !self.empty(r+sign, c-1) && self.get_player(r, c)!=self.get_player(r+sign, c-1){ valid_pos.push((r+sign, c-1)); }
    
        return valid_pos;
        // en passant  
    }
    
    pub fn valid_moves(&mut self, r: i32, c: i32) -> Vec<(i32, i32)>{
        let mut valid_pos: Vec<(i32, i32)> = Vec::new();
        let mut add: Vec<(i32, i32)> = Vec::new();
        let mut range = 0;
    
        let piece = self.get_piece(r, c); let player = self.get_player(r, c);
    
        // note what color
        if piece == "pawn".to_string() {
            return self.pawn_valid_moves(r, c);
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
                if !self.in_range(new_r, new_c) {break;}
    
                if self.get_piece(new_r, new_c) != "empty".to_string() {
                    if player != self.get_player(new_r, new_c) { valid_pos.push((new_r, new_c)); }
                    break;
                }
                valid_pos.push((new_r, new_c));
            }
        }
    
        return valid_pos;
    }
}