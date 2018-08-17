
use std::cmp::Ord;
use std::mem;

use slice::Slice;
use index::Index;

use super::insertion_sort;

const QSORT_THRESHOLD: usize = 16;

pub fn sort<T: Ord>(slice: &mut Slice<T>) {
    if slice.len() < QSORT_THRESHOLD {
        return insertion_sort::sort(slice);
    }

    let sep = part_left(slice);

    let (mut left, _, mut right) = slice.split_tri(sep);

    if let Some(left) = &mut left {
        sort(left);
    }

    if let Some(right) = &mut right {
        sort(right);
    }
}

fn part_left<T: Ord>(slice: &mut Slice<T>) -> Index {
    let pivot = slice.last_idx();

    for idx in slice.iter() {
        let cmp = slice.apply(idx, pivot, swap_on_rev);

        if cmp == Some(true) {
            let (_, mut unparted) = slice.split_right(idx);
            let subidx = part_right(&mut unparted);
            return unparted.parent_idx(subidx);
        }
    }

    pivot
}

fn part_right<T: Ord>(slice: &mut Slice<T>) -> Index {
    let pivot = slice.first_idx();

    for idx in slice.iter_rev() {
        let cmp = slice.apply(pivot, idx, swap_on_rev);

        if cmp == Some(true) {
            let (mut unparted, _) = slice.split_left(idx);
            let subidx = part_left(&mut unparted);
            return unparted.parent_idx(subidx);
        }
    }

    pivot
}

fn swap_on_rev<T: Ord>(left: &mut T, right: &mut T) -> bool {
    if *left > *right {
        mem::swap(left, right);
        true
    } else {
        false
    }
}
