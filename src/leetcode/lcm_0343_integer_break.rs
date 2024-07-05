/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[2] = 1;

        for i in 3..n + 1 {
            for j in 1..i - 1 {
                dp[i] = cmp::max(dp[i], cmp::max(dp[i - j] * j, (i - j) * j));
            }
        }

        dp[n] as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::integer_break(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::integer_break(10), 36);
    }
}
