/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k);

        nums.into_iter().for_each(|item| {
            *map.entry(item).or_insert(0) += 1;
        });

        for (k, v) in map {
            if heap.len() == heap.capacity() {
                if *heap.peek().unwrap() < (Reverse(v), k) {
                    continue;
                } else {
                    heap.pop();
                }
            }
            heap.push((Reverse(v), k));
        }

        heap.into_iter().map(|(_, k)| k).collect()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![2, 1]
        );
    }
}
