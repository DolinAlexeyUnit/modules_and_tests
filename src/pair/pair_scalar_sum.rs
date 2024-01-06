use crate::pair::pair_vector_sum::Pair;

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(pair_scalar_sum((872, 962), (772, 72)), 2678);
    }
}