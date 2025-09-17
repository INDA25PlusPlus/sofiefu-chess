// fn show_board(){
//     println!("o");
//     for n in 1..20{
//         println!("-");
//     }

//     for i in 1..20{
//         println!("-");
//     }
use crate::reset::game_state; 

pub fn show_square(r: i32, c: i32, game: &mut game_state){ // takes two integers [1,8] as input

    let ur = r as usize; let uc = c as usize;
    print!("{} ", game.get_player(r, c));
    println!("{}", game.get_piece(r, c));
}