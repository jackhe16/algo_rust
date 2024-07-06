//! # 线段树
//!
//! ## 应用场景
//! - 求区间和
//!

use std::ops::{Add, Range};

pub struct SegmentTree<T> {
    len: usize,
    tree: Vec<T>,
}

impl<T> From<&Vec<T>> for SegmentTree<T>
where
    T: Default + Copy + Add<Output = T>,
{
    fn from(value: &Vec<T>) -> Self {
        let len = value.len();
        let mut tree = vec![T::default(); 2 * len];
        tree[len..2 * len].clone_from_slice(value);
        for i in (1..len).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        SegmentTree { len, tree }
    }
}

impl<T> SegmentTree<T>
where
    T: Add<Output = T> + Copy,
{
    /// 区间和
    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let mut l = range.start + self.len;
        let mut r = range.end.min(self.len) + self.len;
        let mut res = None;
        while l < r {
            if l % 2 == 1 {
                res = Some(match res {
                    None => self.tree[l],
                    Some(v) => v + self.tree[l],
                });
                l += 1;
            }

            if r % 2 == 1 {
                r -= 1;
                res = Some(match res {
                    None => self.tree[r],
                    Some(v) => v + self.tree[r],
                })
            }

            l /= 2;
            r /= 2;
        }
        res
    }

    /// 更新值
    pub fn update(&mut self, idx: usize, val: T) {
        if idx >= self.len {
            return;
        }

        let mut idx = idx + self.len;
        self.tree[idx] = val;
        idx /= 2;
        while idx != 0 {
            self.tree[idx] = self.tree[2 * idx] + self.tree[2 * idx + 1];
            idx /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let st = SegmentTree::from(&vec![1, 2, 3, 4, 5]);
        assert_eq!(st.query(2..5), Some(12));
    }

    #[test]
    fn test_update() {
        let mut st = SegmentTree::from(&vec![1, 2, 3, 4, 5]);
        st.update(2, 6);
        assert_eq!(st.query(2..5), Some(15));
    }
}
