use crate::pair::pair_vector_sum::Pair;
pub fn default_pair() -> Pair {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(default_pair(), (0, 0));
    }
}