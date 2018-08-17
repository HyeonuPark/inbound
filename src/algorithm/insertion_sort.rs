
use std::mem;
use std::cmp::Ord;

use itertools::Itertools;

use slice::Slice;

pub fn sort<T: Ord>(slice: &mut Slice<T>) {
    for idx in slice.iter() {
        let (mut slice, _) = slice.split_left(idx);

        for (right, left) in slice.iter_rev().tuple_windows() {
            let done = slice.apply(right, left, |right, left| {
                if right < left {
                    mem::swap(right, left);
                    false
                } else {
                    true
                }
            });

            if done == Some(true) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut slice = [5, 8, 7, 7, 4];
        let mut slice = Slice::new(&mut slice).unwrap();
        sort(&mut slice);
        assert_eq!(slice.as_slice(), &[4, 5, 7, 7, 8]);
    }
}
