//! A structure that holds the last N items pushed to it.
//! It is a wrapper around the standard vectors.
//! Read [this](https://github.com/david-soto-m/sliding_window_alt/blob/main/README.md) for more context.
#[doc = include_str!("../README.md")]
use std::iter::{Chain, Iterator};
use std::ops::Index;
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

/// Consts for panics in the crate
mod panics {
    /// Used in the Froms to panic when you try to start a zero width buffer
    pub const START_EMPTY: &str = "can't operate on empty SlidingWindow";
}

#[cfg(test)]
mod tests {
    use crate::SlidingWindow;
    #[test]
    fn create() {
        let st = SlidingWindow::new(5, 6);
        assert_eq!(st, [6; 5]);
    }
    #[test]
    fn create_too_small() {
        let st = SlidingWindow::new(0, 6);
        assert_eq!(st, [6; 1]);
    }
    #[test]
    fn from() {
        let a = [5, 5, 5];
        let st = SlidingWindow::<u8>::from(a);
        assert_eq!(st, a);
    }
    #[test]
    fn from_slice() {
        let a = vec![5, 5, 5];
        let st = SlidingWindow::from(&a[..]);
        assert_eq!(st, &a[..]);
    }
    #[test]
    #[should_panic(expected = "can't operate on empty SlidingWindow")]
    fn from_empty() {
        let a = [];
        let _ = SlidingWindow::<u8>::from(a);
    }
    #[test]
    fn into_arr() {
        let st: SlidingWindow<u8> = [1, 1, 2].into();
        assert_eq!(st, [1, 1, 2])
    }
    #[test]
    fn into_vec() {
        let a = vec![1, 1, 2];
        let st: SlidingWindow<u8> = a[..].into();
        assert_eq!(st, &a[..])
    }
    #[test]
    #[should_panic(expected = "can't operate on empty SlidingWindow")]
    fn into_empty() {
        let _: SlidingWindow<u8> = [].into();
    }
    #[test]
    fn push() {
        let mut st = SlidingWindow::from([10, 15, 16]);
        assert_eq!(st, [10, 15, 16]);
        st.push(5);
        assert_eq!(st, [5, 10, 15]);
        st.push(6);
        assert_eq!(st, [6, 5, 10]);
        st.push(7);
        assert_eq!(st, [7, 6, 5]);
        st.push(8);
        assert_eq!(st, [8, 7, 6]);
    }
    #[test]
    fn push_slice() {
        let mut st = SlidingWindow::from([10, 15, 16]);
        st.push_slice(&[3, 2, 1]);
        assert_eq!(st, [3, 2, 1]);
        st.push_slice(&[2, 3]);
        assert_eq!(st, [2, 3, 3]);
        st.push_slice(&[4]);
        assert_eq!(st, [4, 2, 3]);
    }
    #[test]
    fn push_slice_too_much() {
        let mut st = SlidingWindow::new(5, 6);
        let a = [1; 6];
        st.push_slice(&a);
        assert_ne!(st, a);
        assert_eq!(st, [1; 5]);
    }
    #[test]
    fn use_iter() {
        let a = [1, 2, 3, 4];
        let mut st = SlidingWindow::from(a);
        st.push(2);
        let a = [2, 1, 2, 3];
        // println!("{:?}, {:?}",st, a);
        // internally the vector is different, if its not correctly reordered
        // this will fail
        st.iter().zip(a).for_each(|(i, j)| {
            assert_eq!(*i, j);
        });
    }
    #[test]
    fn use_iter_no_modif() {
        let a = [1, 2, 3, 4];
        let mut st = SlidingWindow::from(a);
        st.iter().map(|x| x * x).count();
        assert!(st.iter().zip(a).all(|(x, y)| { *x == y }));
        st.iter_mut().map(|x| *x *= *x).count();
        st.iter().zip(a).for_each(|(x, y)| {
            assert_eq!(*x, y * y);
        })
    }
    #[test]
    fn use_iter_mut() {
        let a = [1, 2, 3, 4];
        let mut st = SlidingWindow::from(a);
        st.push(2);
        st.iter_mut().for_each(|s| *s *= *s);
        let a: Vec<u8> = [2, 1, 2, 3].iter().map(|i| i * i).collect();
        assert_eq!(st, &a[..]);
    }
    #[test]
    fn for_loop() {
        let a = [1, 2, 3, 4];
        let st = SlidingWindow::from(a);
        let mut b = Vec::<i32>::new();
        for c in &st {
            b.push(c);
        }
        assert_eq!(st, &b[..]);
    }
    #[test]
    fn index() {
        let a = [1, 2, 3, 4, 5];
        let mut st = SlidingWindow::from(a);
        assert!(a
            .iter()
            .enumerate()
            .all(|(index, value)| { *value == st[index] }));
        st.push(5);
        let a = [5, 1, 2, 3, 4];
        assert!(a
            .iter()
            .enumerate()
            .all(|(index, value)| { *value == st[index] }));
        st.push_slice(&[5, 3]);
        let a = [5, 3, 5, 1, 2];
        a.iter().enumerate().for_each(|(index, value)| {
            assert_eq!(*value, st[index]);
        });
    }
    #[test]
    fn partialeqs() {
        let a = [5; 4];
        let st = SlidingWindow::new(4, 5);
        assert_eq!(st, a);
        let a = vec![1, 5, 6, 7, -8];
        let st = SlidingWindow::from(&a[..]);
        assert_eq!(st, &a[..]);
    }
    #[test]
    fn partialneqs() {
        // longer
        let a = [5; 5];
        let st = SlidingWindow::new(4, 5);
        assert_ne!(st, a);
        let mut a = vec![1, 5, 6, -7, 8];
        let st = SlidingWindow::from(&a[..]);
        a.push(5);
        assert_ne!(st, &a[..]);
        // Shorter
        let a = [5; 3];
        let st = SlidingWindow::new(4, 5);
        assert_ne!(st, a);
        let mut a = vec![1, 5, 6, 7, 8];
        let st = SlidingWindow::from(&a[..]);
        a.pop();
        assert_ne!(st, &a[..]);
        // different
        let a = [5, -5, 5, 3];
        let st = SlidingWindow::new(4, 5);
        assert_ne!(st, a);
        let mut a = vec![1, 5, 6, 7, 8];
        let st = SlidingWindow::from(&a[..]);
        a.pop();
        a.push(4);
        assert_ne!(st, &a[..]);
    }
}

/// A structure that holds the last N items pushed to it.
#[derive(Debug)]
pub struct SlidingWindow<T>
where
    T: Clone,
{
    vec: Vec<T>,
    current_insert: usize,
    capacity: usize,
}

use std::fmt::Debug;
impl<T> SlidingWindow<T>
where
    T: Clone,
{
    /// The first item is the capacity of the sliding window and the second the
    /// initial value of all the elements. **At creation, all elements are set.**
    pub fn new(mut max_items: usize, init: T) -> SlidingWindow<T> {
        if max_items < 1 {
            max_items = 1;
        }
        SlidingWindow {
            vec: vec![init; max_items],
            current_insert: 0,
            capacity: max_items,
        }
    }
    /// Push an element to the window, forgetting the oldest.
    pub fn push(&mut self, a: T) {
        self.vec[self.capacity - 1 - self.current_insert] = a;
        self.current_insert += 1;
        self.current_insert %= self.capacity;
    }
    /// Push a slice, where the newest item is at index 0.
    pub fn push_slice(&mut self, a: &[T]) {
        a.iter().rev().for_each(|a| {
            self.push(a.to_owned());
        });
    }

    /// Returns the total capacity of the sliding window, though you should know
    /// it, as there is a fixed size since creation.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn splitter(&self) -> usize {
        self.capacity - self.current_insert
    }

    /// Returns an ordered iterator, where the first element is the newest and
    /// the last, the oldest.
    pub fn iter(&self) -> Chain<Iter<T>, Iter<T>> {
        // it doesn't rely on as_vec, bcs this way is lazier
        let (a, b) = self.vec.split_at(self.splitter());
        b.iter().chain(a.iter())
    }

    /// Returns a mutable iterator in the same order as the `iter` method.
    pub fn iter_mut(&mut self) -> Chain<IterMut<T>, IterMut<T>> {
        let (a, b) = self.vec.split_at_mut(self.capacity - self.current_insert);
        b.iter_mut().chain(a.iter_mut())
    }

    /// Returns a vector that starts at the newest element.
    pub fn to_vec(&self) -> Vec<T> {
        let (a, b) = self.vec.split_at(self.splitter());
        [b, a].concat()
    }
}

impl<T: Clone> IntoIterator for SlidingWindow<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<T: Clone> IntoIterator for &SlidingWindow<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}

impl<T: Clone> FromIterator<T> for SlidingWindow<T> {
    fn from_iter<Q: IntoIterator<Item = T>>(iter: Q) -> Self {
        let vec: Vec<T> = iter.into_iter().collect();
        let capacity = vec.len();
        SlidingWindow {
            vec,
            current_insert: 0,
            capacity,
        }
    }
}

impl<T: Clone, const LEN: usize> From<[T; LEN]> for SlidingWindow<T> {
    fn from(a: [T; LEN]) -> Self {
        if a.is_empty() {
            panic!("{}", panics::START_EMPTY);
        }
        SlidingWindow {
            vec: a.to_vec(),
            current_insert: 0,
            capacity: a.len(),
        }
    }
}

impl<T: Clone> From<&[T]> for SlidingWindow<T> {
    fn from(a: &[T]) -> Self {
        if a.is_empty() {
            panic!("{}", panics::START_EMPTY);
        }
        SlidingWindow {
            vec: a.to_vec(),
            current_insert: 0,
            capacity: a.len(),
        }
    }
}

impl<T: Clone + PartialEq> PartialEq for SlidingWindow<T> {
    fn eq(&self, other: &Self) -> bool {
        self.capacity == other.capacity && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: Clone + PartialEq> PartialEq<&[T]> for SlidingWindow<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.capacity == other.len() && self.iter().zip(*other).all(|(x, y)| x == y)
    }
}

impl<T, const LEN: usize> PartialEq<[T; LEN]> for SlidingWindow<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &[T; LEN]) -> bool {
        self.capacity == LEN && self.iter().zip(other.iter()).all(|(x, y)| x == y)
    }
}

impl<T: Clone> Index<usize> for SlidingWindow<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[(self.capacity - self.current_insert + index) % self.capacity]
    }
}
