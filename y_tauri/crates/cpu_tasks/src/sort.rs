use rayon::prelude::*;

pub fn sort_seq(v: &mut [u64]) {
    v.sort_unstable();
}

pub fn sort_par(v: &mut [u64]) {
    v.par_sort_unstable();
}
