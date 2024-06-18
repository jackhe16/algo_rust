/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

use std::collections::VecDeque;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut ans = Vec::with_capacity(n - k + 1);
        let mut q = VecDeque::new();

        for (i, &item) in nums.iter().enumerate() {
            while !q.is_empty() && nums[*q.back().unwrap()] <= item {
                q.pop_back();
            }
            q.push_back(i);
            if i - q[0] >= k {
                q.pop_front();
            }
            if i >= k - 1 {
                ans.push(nums[q[0]])
            }
        }

        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
