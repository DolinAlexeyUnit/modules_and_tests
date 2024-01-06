use modules_and_tests::pair::default_pair::default_pair;
use modules_and_tests::pair::pair_scalar_sum::pair_scalar_sum;
use modules_and_tests::pair::pair_vector_sum::pair_vector_sum;
use modules_and_tests::signed::default_signed_counter::default_signed_counter;
use modules_and_tests::signed::next_signed::next_signed;
use modules_and_tests::signed::prev_signed::prev_signed;
use modules_and_tests::unsigned::default_unsigned_counter::default_unsigned_counter;
use modules_and_tests::unsigned::next_unsigned::next_unsigned;
use modules_and_tests::vec3::default_vec3::default_vec3;
use modules_and_tests::vec3::vec3_scalar_sum::vec3_scalar_sum;
use modules_and_tests::vec3::vec3_vector_sum::vec3_vector_sum;

fn main() {
    default_signed_counter();
    default_unsigned_counter();
    default_vec3();
    default_pair();
    next_signed(1);
    next_unsigned(2);
    prev_signed(3);
    vec3_vector_sum([1, 2, 3], [4, 5, 6]);
    pair_vector_sum([6, 7].into(), [8, 9].into());
    vec3_scalar_sum([12, 13, 38], [14, 15, 42]);
    pair_scalar_sum([16, 17].into(), [18, 19].into());
}
