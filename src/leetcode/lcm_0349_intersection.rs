/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set: HashSet<i32> = nums1.into_iter().collect();
        let mut result_set = HashSet::with_capacity(1000);
        for num in nums2.iter() {
            if nums1_set.contains(num) {
                result_set.insert(*num);
            }
        }

        let ret: Vec<i32> = result_set.into_iter().collect();
        ret
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
    }
}
