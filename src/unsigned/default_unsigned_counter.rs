use crate::unsigned::next_unsigned::UnsignedCounter;

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(default_unsigned_counter(), 0);
    }
}