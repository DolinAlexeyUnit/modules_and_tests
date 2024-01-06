use crate::vec3::vec3_vector_sum::VEC3_LEN;
pub type Vec3 = [i32; VEC3_LEN];

pub fn default_vec3() -> Vec3 {
    [0; 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(default_vec3(), [0, 0, 0]);
    }
}