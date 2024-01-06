use crate::signed::prev_signed::SignedCounter;

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(next_signed(1), 2);
        assert_eq!(next_signed(99999999999999), 100000000000000);
        assert_eq!(next_signed(-99999999999999), -99999999999998);
     }
}