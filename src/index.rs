
use std::cmp::{PartialEq, PartialOrd, Ord, Ordering};
use std::iter::DoubleEndedIterator;

use slice::Slice;

#[cfg(debug_assertions)]
use puid;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Index {
    pub(crate) value: usize,
    #[cfg(debug_assertions)]
    pub(crate) tag: puid::Id,
}

#[derive(Debug)]
pub struct Iter {
    length: usize,
    current: usize,
    #[cfg(debug_assertions)]
    tag: puid::Id,
}

impl PartialEq for Index {
    fn eq(&self, rhs: &Self) -> bool {
        #[cfg(debug_assertions)]
        assert_eq!(self.tag, rhs.tag);

        PartialEq::eq(&self.value, &rhs.value)
    }
}

impl Ord for Index {
    fn cmp(&self, rhs: &Self) -> Ordering {
        #[cfg(debug_assertions)]
        assert_eq!(self.tag, rhs.tag);

        Ord::cmp(&self.value, &rhs.value)
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        #[cfg(debug_assertions)]
        assert_eq!(self.tag, rhs.tag);

        PartialOrd::partial_cmp(&self.value, &rhs.value)
    }
}

impl Iter {
    pub(crate) fn new<T>(slice: &Slice<T>, current: usize) -> Self {
        debug_assert!(current <= slice.len());
        
        Iter {
            length: slice.len(),
            current,
            #[cfg(debug_assertions)]
            tag: slice.tag,
        }
    }
}

impl Iterator for Iter {
    type Item = Index;

    fn next(&mut self) -> Option<Index> {
        if self.current == self.length {
            None
        } else {
            let value = self.current;
            self.current += 1;

            Some(Index {
                value,
                #[cfg(debug_assertions)]
                tag: self.tag,
            })
        }
    }
}

impl DoubleEndedIterator for Iter {
    fn next_back(&mut self) -> Option<Index> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;

            Some(Index {
                value: self.current,
                #[cfg(debug_assertions)]
                tag: self.tag,
            })
        }
    }
}
