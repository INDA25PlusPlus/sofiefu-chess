
mod reset;
mod play;
mod visualize;

use chess::reset::reset_board; // use statements are shortcuts to avoid typing full path every time
use chess::play::make_move;
use chess::visualize::show_square;
use reset::{BOARD, COLOR};

fn main(){
    println!("Initializing chess board");
    reset_board();
    println!("Board reset");
    show_square(1, 4);
    println!("{}", make_move(2, 2, 4, 2));
    show_square(4, 2);
    // show_sqaure()
    // if ret == 1 {println!("Piece moved");}
}

// perf test

// implement: checkmate, pawn diagonal, rokad, pawn change, tests