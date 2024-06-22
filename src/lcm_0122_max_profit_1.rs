/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut dp = vec![vec![0; len]; 2];
        dp[0][0] = -prices[0];
        dp[1][0] = 0;

        for i in 1..len {
            dp[0][i] = cmp::max(dp[0][i - 1], dp[1][i - 1] - prices[i]);
            dp[1][i] = cmp::max(dp[1][i - 1], dp[0][i - 1] + prices[i]);
        }

        dp[1][len - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
