use rand::prelude::*;

pub(crate) fn gen_rand_arr<const SIZE: usize>(rng: &mut ThreadRng, n: i32) -> [i32; SIZE] {
    let mut arr = [0; SIZE];
    for x in &mut arr {
        *x = rng.gen_range(1..n);
    }
    arr
}
