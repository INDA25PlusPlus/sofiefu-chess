// fn show_board(){
//     println!("o");
//     for n in 1..20{
//         println!("-");
//     }

//     for i in 1..20{
//         println!("-");
//     }
use crate::reset::{BOARD, COLOR}; 

pub fn show_square(r: i32, c: i32){ // takes two integers [1,8] as input
    let board = BOARD.lock().unwrap();
    let color = COLOR.lock().unwrap();

    let ur = r as usize; let uc = c as usize;
    print!("{} ", color[ur][uc]);
    println!("{}", &board[ur][uc]);
}