/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut dp = vec![vec![0; len]; 4];

        dp[0][0] = -prices[0];
        dp[2][0] = -prices[0];

        for i in 1..len {
            dp[0][i] = cmp::max(dp[0][i - 1], -prices[i]);
            dp[1][i] = cmp::max(dp[1][i - 1], dp[0][i - 1] + prices[i]);
            dp[2][i] = cmp::max(dp[2][i - 1], dp[1][i - 1] - prices[i]);
            dp[3][i] = cmp::max(dp[3][i - 1], dp[2][i - 1] + prices[i]);
        }

        dp[3][len - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}
