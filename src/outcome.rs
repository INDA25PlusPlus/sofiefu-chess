#[derive(PartialEq)]
pub enum Outcome {
    // valid moves
    Valid,
    Check,
    Checkmate,
    // invalid moves
    Bad_coordinates, 
    Wrong_player,
    Invalid,
    Checked
}
