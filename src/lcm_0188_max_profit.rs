/*
 * @lc app=leetcode.cn id=188 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let k = k as usize;
        let mut dp = vec![vec![0; len]; k * 2 + 1];

        for i in (0..k * 2).filter(|a| a % 2 == 1) {
            dp[i][0] = -prices[0];
        }

        for j in 1..len {
            for i in 1..k * 2 + 1 {
                if i as i32 % 2 == 1 {
                    dp[i][j] = cmp::max(dp[i][j - 1], dp[i - 1][j - 1] - prices[j]);
                } else {
                    dp[i][j] = cmp::max(dp[i][j - 1], dp[i - 1][j - 1] + prices[j]);
                }
            }
        }

        dp[k * 2][len - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    }
}
