use crate::vec3::default_vec3::default_vec3;

pub const VEC3_LEN: usize = 3;
pub type Vec3 = [i32; VEC3_LEN];

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c = default_vec3();
    for i in 0..3 {
        c[i] = a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec3_vector_sum([1, 2, 3], [4, 5, 6]), [5, 7, 9]);
        assert_eq!(vec3_vector_sum([345, 584, 285], [284, 293, 194]), [629, 877, 479]);
    }
}