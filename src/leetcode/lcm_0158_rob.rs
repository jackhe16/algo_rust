/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return nums[0];
        }

        let mut dp = vec![0; len];
        dp[0] = nums[0];
        dp[1] = cmp::max(nums[0], nums[1]);

        for i in 2..len {
            dp[i] = cmp::max(dp[i - 2] + nums[i], dp[i - 1]);
        }

        dp[len - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
