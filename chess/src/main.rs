
mod reset;
mod play;
mod visualize;

// use statements are shortcuts to avoid typing full path every time
use chess::reset::game_state;
use chess::play::make_move;
use chess::visualize::show_square;

fn main(){
    println!("Initializing chess board");
    let mut game = game_state::new();
    println!("Board created");
    show_square(1, 4, &mut game);
    println!("{}", make_move(2, 2, 4, 2, &mut game));
    show_square(4, 2, & mut game);
    // show_sqaure()
    // if ret == 1 {println!("Piece moved");}
}

// perf test

// implement: checkmate, pawn diagonal, rokad, pawn change, tests