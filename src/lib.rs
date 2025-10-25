
pub mod game;
pub mod visualize; // if these lines not included, these modules not usable because this file decides the library
pub mod outcome;


// TESTING 
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_board() {
        let mut game = game::game_state::new();
    }

    #[test]
    fn test_good_move() {
        let mut game = game::game_state::new();

        let white_move = game.make_move(2, 4, 4, 4); //pawn
        assert!(white_move == outcome::Outcome::Valid);

        let black_move = game.make_move(7, 5, 5, 5); //pawn
        assert!(black_move == outcome::Outcome::Valid);
    }

    #[test]
    fn test_bad_move() {
        let mut game = game::game_state::new();

        let white_failed_move = game.make_move(2, 4, 3, 5); //pawn diagonal
        assert!(white_failed_move == outcome::Outcome::Invalid);

        let white_move = game.make_move(1, 2, 3, 3); //horse
        assert!(white_move == outcome::Outcome::Valid);

        let black_failed_move = game.make_move(8, 2, 7, 4); //horse to own pawn
        assert!(black_failed_move == outcome::Outcome::Invalid);
    }

    #[test]
    fn test_wrongturn() {
        let mut game = game::game_state::new();
        let black_move = game.make_move(7, 2, 5, 2);
        assert!(black_move == outcome::Outcome::Wrong_player);
    }   

    #[test]
    fn test_checkmate() {
        let mut game = game::game_state::new();

        // black checkmates - fools mate
        let white1 = game.make_move(2, 6, 3, 6);
        let black1 = game.make_move(7, 5, 5, 5);
        let white2 = game.make_move(2, 7, 4, 7);
        let black2 = game.make_move(8, 4, 4, 8);
        assert!(black2 == outcome::Outcome::Checkmate);
    }
}



