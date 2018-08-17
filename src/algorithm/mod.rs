
pub mod quick_sort;
pub mod insertion_sort;

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use slice::Slice;

    const ITER_COUNT: usize = 16;

    fn test_sorter(sort: fn(&mut Slice<u32>)) {
        let mut rng = thread_rng();
        let s1 = &mut [0u32; 1024][..];
        let s2 = &mut [0u32; 1024][..];

        for _ in 0..ITER_COUNT {
            rng.fill(s1);
            s2.copy_from_slice(s1);

            s1.sort_unstable();
            sort(&mut Slice::new(s2).unwrap());

            assert_eq!(s1, s2);
        }
    }

    #[test]
    fn test_insertion_sort() {
        test_sorter(super::insertion_sort::sort::<u32>);
    }

    #[test]
    fn test_quick_sort() {
        test_sorter(super::quick_sort::sort::<u32>);
    }
}
