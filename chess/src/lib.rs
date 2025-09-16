
pub mod reset;
pub mod play;
pub mod visualize; // if these lines not included, these modules not usable because this file decides the library


// TESTING 
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


/*
IDEA
struct for each type of piece - instances of structs
each position on the board stores which piece (or none) it contains
1: pawn
2: horse
3: löpare
4: tower
5: queen
6: king
*/

