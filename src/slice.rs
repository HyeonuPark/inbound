
use std::{ops, slice};
use std::iter::Rev;

use index::{Index, Iter};

#[cfg(debug_assertions)]
use puid;

#[derive(Debug)]
pub struct Slice<'a, T: 'a> {
    value: &'a mut [T],
    parent: Option<Parent>,
    #[cfg(debug_assertions)]
    pub(crate) tag: puid::Id,
}

#[derive(Debug, Clone, Copy)]
struct Parent {
    offset: usize,
    #[cfg(debug_assertions)]
    pub(crate) tag: puid::Id,
}

impl<'a, T> ops::Index<Index> for Slice<'a, T> {
    type Output = T;

    fn index(&self, idx: Index) -> &T {
        self.check(idx);

        unsafe {
            self.value.get_unchecked(idx.value)
        }
    }
}

impl<'a,  T> ops::IndexMut<Index> for Slice<'a, T> {
    fn index_mut(&mut self, idx: Index) -> &mut T {
        self.check(idx);

        unsafe {
            self.value.get_unchecked_mut(idx.value)
        }
    }
}

impl<'a, T> Slice<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Option<Self> {
        if slice.is_empty() {
            None
        } else {
            Some(unsafe { Self::new_unchecked(slice) })
        }
    }

    pub unsafe fn new_unchecked(slice: &'a mut [T]) -> Self {
        Slice {
            value: slice,
            parent: None,
            #[cfg(debug_assertions)]
            tag: puid::Id::new(),
        }
    }

    unsafe fn subslice_unchecked<'b>(&mut self, offset: Index, length: usize) -> Slice<'b, T> {
        let head_ptr = &mut self[offset] as *mut T;
        let slice = slice::from_raw_parts_mut(head_ptr, length);
        Slice {
            value: slice,
            parent: Some(Parent {
                offset: offset.value,
                #[cfg(debug_assertions)]
                tag: self.tag,
            }),
            #[cfg(debug_assertions)]
            tag: puid::Id::new(),
        }
    }

    #[inline(always)]
    fn index_of(&self, value: usize) -> Index {
        Index {
            value,
            #[cfg(debug_assertions)]
            tag: self.tag,
        }
    }

    #[inline(always)]
    fn check(&self, _idx: Index) {
        #[cfg(debug_assertions)]
        assert_eq!(self.tag, _idx.tag);
    }

    /// Translate this slice's index to parent's index. No-op if it doesn't have any parent.
    pub fn parent_idx(&self, idx: Index) -> Index {
        self.parent.map_or(idx, |parent| {
            Index {
                value: idx.value + parent.offset,
                #[cfg(debug_assertions)]
                tag: parent.tag,
            }
        })
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn as_slice(&self) -> &[T] {
        &self.value
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.value
    }

    pub fn apply<F, U>(&mut self, left: Index, right: Index, f: F) -> Option<U> where
        F: FnOnce(&mut T, &mut T) -> U
    {
        if left == right {
            None
        } else {
            let (left, right) = unsafe {(
                &mut *(&mut self[left] as *mut T),
                &mut *(&mut self[right] as *mut T),
            )};
            Some(f(left, right))
        }
    }

    pub fn into_first(self) -> &'a mut T {
        unsafe { self.value.get_unchecked_mut(0) }
    }

    pub fn first_idx(&self) -> Index {
        self.index_of(0)
    }

    pub fn last_idx(&self) -> Index {
        self.index_of(self.len() - 1)
    }

    pub fn middle_idx(&self) -> Index {
        self.index_of(self.len() / 2)
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self, 0)
    }

    pub fn iter_rev(&self) -> Rev<Iter> {
        Iter::new(self, self.len()).rev()
    }

    pub fn first(&mut self) -> &mut T {
        let idx = self.first_idx();
        &mut self[idx]
    }

    pub fn last(&mut self) -> &mut T {
        let idx = self.last_idx();
        &mut self[idx]
    }

    pub fn middle(&mut self) -> &mut T {
        let idx = self.middle_idx();
        &mut self[idx]
    }

    pub fn split_left(&mut self, point: Index) -> (Self, Option<Self>) {
        self.check(point);

        let left = unsafe {
            let offset = self.first_idx();
            let length = point.value + 1;
            self.subslice_unchecked(offset, length)
        };

        let right = {
            if self.len() == point.value + 1 {
                None
            } else {
                let offset = self.index_of(point.value + 1);
                let length = self.len() - point.value - 1;

                Some(unsafe {
                    self.subslice_unchecked(offset, length)
                })
            }
        };

        (left, right)
    }

    pub fn split_right(&mut self, point: Index) -> (Option<Self>, Self) {
        self.check(point);

        let left = {
            if point.value == 0 {
                None
            } else {
                let offset = self.first_idx();
                let length = point.value;

                Some(unsafe {
                    self.subslice_unchecked(offset, length)
                })
            }
        };

        let right = {
            let offset = point;
            let length = self.len() - point.value;

            unsafe {
                self.subslice_unchecked(offset, length)
            }
        };

        (left, right)
    }

    pub fn split_tri(&mut self, point: Index) -> (Option<Self>, &mut T, Option<Self>) {
        self.check(point);

        let left = {
            if point.value == 0 {
                None
            } else {
                let offset = self.first_idx();
                let length = point.value;

                Some(unsafe {
                    self.subslice_unchecked(offset, length)
                })
            }
        };

        let right = {
            if self.len() == point.value + 1 {
                None
            } else {
                let offset = self.index_of(point.value + 1);
                let length = self.len() - point.value - 1;

                Some(unsafe {
                    self.subslice_unchecked(offset, length)
                })
            }
        };

        let mid = &mut self[point];

        (left, mid, right)
    }

    pub fn split_first(&mut self) -> (&mut T, Option<Self>) {
        let idx = self.first_idx();
        let (head, tail) = self.split_left(idx);
        (head.into_first(), tail)
    }

    pub fn split_last(&mut self) -> (Option<Self>, &mut T) {
        let idx = self.last_idx();
        let (head, tail) = self.split_right(idx);
        (head, tail.into_first())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(debug_assertions, should_panic)]
    fn test_foreign_index() {
        let mut a1 = [1, 2, 3u32];
        let mut a2 = [4, 5, 6u32];

        let s1 = Slice::new(&mut a1[..]).unwrap();
        let mut s2 = Slice::new(&mut a2[..]).unwrap();

        let _ = &s1[s2.first_idx()];
        let _ = &mut s2[s1.last_idx()];
    }
}
