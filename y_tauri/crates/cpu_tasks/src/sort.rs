use std::cmp::Ordering;

use rayon::prelude::*;

pub fn sort_seq(v: &mut [u64]) {
    v.sort_unstable();
}

pub fn quick_sort_par(v: &mut [u64]) {
    if v.len() <= SEQUENTIAL_THRESHOLD {
        v.sort_unstable();
        return;
    }
    let (lt, gt) = partition3(v);
    let (left, rest) = v.split_at_mut(lt);
    let (_equal, right) = rest.split_at_mut(gt - lt);
    rayon::join(|| quick_sort_par(left), || quick_sort_par(right));
}

const SEQUENTIAL_THRESHOLD: usize = 8 * 1024;

fn partition3<T: Ord>(v: &mut [T]) -> (usize, usize) {
    v.swap(0, v.len() / 2);
    let (mut lt, mut i, mut gt) = (0, 1, v.len());
    while i < gt {
        match v[i].cmp(&v[lt]) {
            Ordering::Less => {
                v.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Greater => {
                gt -= 1;
                v.swap(i, gt);
            }
            Ordering::Equal => i += 1,
        }
    }
    (lt, gt)
}

pub fn sort_par(v: &mut [u64]) {
    v.par_sort_unstable();
}
