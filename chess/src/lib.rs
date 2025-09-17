
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



