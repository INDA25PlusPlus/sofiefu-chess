
mod game;
mod visualize;
mod outcome;

// use statements are shortcuts to avoid typing full path every time
use chess::game::game_state;
use chess::visualize::show_square;
use chess::outcome::Outcome;

fn main(){
    println!("Initializing chess board");
    let mut game = game_state::new();
    println!("Board created");
    // show_square(1, 4, &mut game);
    let result = game.make_move(2, 4, 4, 4);
    if result == Outcome::Valid { println!("Valid"); }
    // show_square(4, 2, &mut game);
}

// TODO: implement checking check and checkmate for rocade, and en passant