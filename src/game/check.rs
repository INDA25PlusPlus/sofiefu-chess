use crate::game::game_state; 

impl game_state {
    pub fn is_checking(&mut self, player: char) -> bool{ // is player=player checking opponent
        for i in 1..9 {
            for u in 1..9 {
                if self.board[i][u] != "empty".to_string() && self.player[i][u]==player {    
                    let ui = i as i32; let uu = u as i32;
                    let valid_pos = self.valid_moves(ui, uu);
                    for &(a, b) in &valid_pos {
                        if self.get_piece(a, b) == "king".to_string() && self.get_player(a, b) != player {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    
    pub fn is_checkmate(&mut self) -> bool{ // if i am checkmating opponent, assuming i am checking
        // find king, if no move 
        let opponent = if self.turn=='w' {'b'} else {'w'};

        for i in 1..9 {
            for u in 1..9 {
                if self.player[i][u]==self.turn {
                    let r = i as i32; let c = u as i32; 

                    let valid_pos = self.valid_moves(r, c);
                    for &(a, b) in &valid_pos {
                        let mut upd_game = self.clone();
                        upd_game.move_piece(r, c, a, b);
                        if !upd_game.is_checking(opponent) {return false;}
                    }
                }
            }
        }
        return true;
    }

}