pub type SignedCounter = isize;

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(prev_signed(10), 9);
    }
}