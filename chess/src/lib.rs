/*
Idea: 
struct for each type of piece - instances of structs
each position on the board stores which piece (or none) it contains
1: pawn
2: horse
3: löpare
4: tower
5: queen
6: king

*/

pub mod piece;



// fn is_move_valid(){

// }

// fn is_checkmate(){

// } 

// fn valid_moves(){
//     // change color
// }
#[derive(Clone)]
pub struct Piece{
    x_pos: char,
    y_pos: usize,
    kind: String,
    color: bool,
}

pub fn reset_board(){
    let mut board: Vec<Vec<Option<Piece>>> = vec![vec![None; 9]; 9];
    let alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for letter in alphabet{
        let x = letter as usize - 96;
        let pawn_white = Piece{
            x_pos: letter,
            y_pos: 2,
            kind: "pawn".to_string(),
            color: true,
        };
        board[2][x] = Some(pawn_white);

        let pawn_black = Piece{
            x_pos: letter,
            y_pos: 7,
            kind: "pawn".to_string(),
            color: false,
        };
        board[7][x] = Some(pawn_black);

    }

    let rows = vec![8, 1];
    for row in rows{
        let c: bool;

        if row == 8 {c=false;}
        else {c=true;}

        board[row][1] = Some(Piece{x_pos:'a', y_pos:row, kind: "rook".to_string(), color: c});
        board[row][8] = Some(Piece{x_pos:'h', y_pos:row, kind: "rook".to_string(), color: c});

        board[row][2] = Some(Piece{x_pos:'b', y_pos:row, kind: "knight".to_string(), color: c});
        board[row][7] = Some(Piece{x_pos:'g', y_pos:row, kind: "knight".to_string(), color: c});

        board[row][3] = Some(Piece{x_pos:'c', y_pos:row, kind: "bishop".to_string(), color: c});
        board[row][6] = Some(Piece{x_pos:'f', y_pos:row, kind: "bishop".to_string(), color: c});

        board[row][4] = Some(Piece{x_pos:'d', y_pos:row, kind: "queen".to_string(), color: c});
        board[row][5] = Some(Piece{x_pos:'e', y_pos:row, kind: "king".to_string(), color: c});
    }

    println!("{}", board[2][2].as_ref().unwrap().kind);
}

fn show_square(x: i32, y: i32){

}

// fn show_board(){
//     println!("o");
//     for n in 1..20{
//         println!("-");
//     }

//     for i in 1..20{
//         println!("-");
//     }


// }

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

