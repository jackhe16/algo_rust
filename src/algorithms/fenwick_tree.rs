//! # 芬威克树(树状数组)
//!
//! ## 应用场景
//! - 前缀和
//!

use std::ops::{Add, AddAssign};

pub struct FenwickTree<T> {
    data: Vec<T>,
}

impl<T: Default + Copy + Add + AddAssign> FenwickTree<T> {
    pub fn with_len(len: usize) -> FenwickTree<T> {
        FenwickTree {
            data: vec![T::default(); len + 1],
        }
    }

    pub fn add(&mut self, i: usize, v: T) {
        let mut i = i + 1;
        while i < self.data.len() {
            self.data[i] += v;
            i += lowbit(i);
        }
    }

    pub fn prefix_sum(&self, i: usize) -> T {
        let mut i = i + 1;
        let mut res = T::default();
        while i > 0 {
            res += self.data[i];
            i -= lowbit(i);
        }
        res
    }
}

const fn lowbit(x: usize) -> usize {
    let x = x as isize;
    (x & (-x)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut ft = FenwickTree::with_len(6);

        ft.add(0, 1);
        ft.add(1, 3);
        ft.add(2, 5);
        ft.add(3, 7);
        ft.add(4, 9);
        ft.add(5, 11);

        assert_eq!(ft.prefix_sum(0), 1);
        assert_eq!(ft.prefix_sum(1), 4);
        assert_eq!(ft.prefix_sum(2), 9);
        assert_eq!(ft.prefix_sum(3), 16);
        assert_eq!(ft.prefix_sum(4), 25);
        assert_eq!(ft.prefix_sum(5), 36);
    }
}
