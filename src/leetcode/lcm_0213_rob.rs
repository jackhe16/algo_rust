/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        match len {
            1 => nums[0],
            _ => cmp::max(
                Self::rob_range(&nums, 0, len - 2),
                Self::rob_range(&nums, 1, len - 1),
            ),
        }
    }

    pub fn rob_range(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        if start == end {
            return nums[start];
        }

        let mut dp = vec![0; nums.len()];
        dp[start] = nums[start];
        dp[start + 1] = cmp::max(nums[start], nums[start + 1]);

        for i in start + 2..=end {
            dp[i] = cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
        }

        dp[end]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
}
