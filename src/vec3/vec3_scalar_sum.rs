use crate::vec3::vec3_vector_sum::Vec3;
use crate::vec3::vec3_vector_sum::VEC3_LEN;
pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec3_scalar_sum([345, 584, 33], [394, 98, 372]), 1826);
    }
}