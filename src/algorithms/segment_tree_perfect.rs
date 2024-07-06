//! # 线段树
//!
//! ## 应用场景
//! - 求区间和
//!
//! ## 复杂度
//! - 求区间和: O(logn)
//! - 更新数据: O(logn)
//!
//! ## 算法思路
//! - 给定原始数据 data[0..n], 构造完美二叉树数组tree[0..2^(k+1)], k表示n-1右移了k次变为0
//! - 假如n=3, 则
//!   - tree[0]代表区间[0..=2]
//!    - tree[1]代表区间[0..=1]
//!      - tree[3]代表区间[0..=0]
//!      - tree[4]代表区间[1..=1]
//!    - tree[2]代表区间[2..=2]
//!      - tree[5]空闲
//!      - tree[6]空闲
//! - 更新操作只会影响祖先节点
//!

use std::ops::{Add, Range};

pub struct SegmentTree<T> {
    data: Vec<T>,
    len: usize,
    tree: Vec<T>,
}

impl<T> From<Vec<T>> for SegmentTree<T>
where
    T: Default + Add<Output = T> + Copy,
{
    fn from(value: Vec<T>) -> Self {
        let len = value.len();

        let mut i = len;
        let mut n = 1;
        while i != 0 {
            i >>= 1;
            n <<= 1;
        }
        n <<= 1;

        let mut tree = vec![T::default(); n];
        recursive_build_tree(&value, &mut tree, 0, 0..len);

        SegmentTree {
            data: value,
            len,
            tree,
        }
    }
}

fn recursive_build_tree<T>(data: &Vec<T>, tree: &mut Vec<T>, ti: usize, data_range: Range<usize>)
where
    T: Add<Output = T> + Copy,
{
    let dl = data_range.start;
    let dr = data_range.end;

    if dl + 1 == dr {
        tree[ti] = data[dl];
        return;
    }

    let dmid = (dl + dr) >> 1;
    let lti = 2 * ti + 1;
    let rti = 2 * ti + 2;

    recursive_build_tree(data, tree, lti, dl..dmid);
    recursive_build_tree(data, tree, rti, dmid..dr);
    tree[ti] = tree[lti] + tree[rti];
}

impl<T> SegmentTree<T>
where
    T: Default + Add<Output = T> + Copy,
{
    /// 区间和
    pub fn query(&self, query_range: Range<usize>) -> T {
        recursive_query(&self.data, &self.tree, 0, 0..self.len, &query_range)
    }

    /// 更新值
    pub fn update(&mut self, di: usize, val: T) {
        recursive_update(&mut self.data, &mut self.tree, 0, 0..self.len, di, val);
    }
}

fn recursive_query<T>(
    data: &Vec<T>,
    tree: &Vec<T>,
    ti: usize,
    data_range: Range<usize>,
    query_range: &Range<usize>,
) -> T
where
    T: Default + Add<Output = T> + Copy,
{
    let dl = data_range.start;
    let dr = data_range.end;
    let ql = query_range.start;
    let qr = query_range.end;

    if ql >= dr || qr <= dl {
        return T::default();
    }
    if ql <= dl && qr >= dr {
        return tree[ti];
    }

    let dmid = (dl + dr) >> 1;
    let lti = 2 * ti + 1;
    let rti = 2 * ti + 2;

    let sum_left = recursive_query(data, tree, lti, dl..dmid, query_range);
    let sum_right = recursive_query(data, tree, rti, dmid..dr, query_range);
    sum_left + sum_right
}

fn recursive_update<T>(
    data: &mut Vec<T>,
    tree: &mut Vec<T>,
    ti: usize,
    data_range: Range<usize>,
    di: usize,
    val: T,
) where
    T: Add<Output = T> + Copy,
{
    let dl = data_range.start;
    let dr = data_range.end;

    if dl + 1 == dr {
        data[di] = val;
        tree[ti] = val;
        return;
    }

    let dmid = (dl + dr) >> 1;
    let lti = 2 * ti + 1;
    let rti = 2 * ti + 2;

    if di < dmid {
        recursive_update(data, tree, lti, dl..dmid, di, val);
    } else {
        recursive_update(data, tree, rti, dmid..dr, di, val);
    }
    tree[ti] = tree[lti] + tree[rti];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let st = SegmentTree::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(st.query(2..5), 12)
    }

    #[test]
    fn test_update() {
        let mut st = SegmentTree::from(vec![1, 2, 3, 4, 5]);
        st.update(2, 6);
        assert_eq!(st.query(2..5), 15);
    }
}
