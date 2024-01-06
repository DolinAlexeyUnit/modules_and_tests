use crate::signed::prev_signed::SignedCounter;

pub fn default_signed_counter() -> SignedCounter {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(default_signed_counter(), 0);
    }
}