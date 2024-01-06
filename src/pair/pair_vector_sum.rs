pub type Pair = (i32, i32);

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(pair_vector_sum((345, 584), (2, 3)), (347, 587));
    }
}