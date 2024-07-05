/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![0; target + 1];
        for n in nums {
            for j in ((n as usize)..target + 1).rev() {
                dp[j] = cmp::max(dp[j], dp[j - n as usize] + n as usize);
            }
        }

        dp[target] == target
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    }
}
