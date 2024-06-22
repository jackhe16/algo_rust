/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;

        for coin in coins {
            let coin = coin as usize;
            for j in coin..amount + 1 {
                if dp[j - coin] != i32::MAX {
                    dp[j] = cmp::min(dp[j], dp[j - coin] + 1);
                }
            }
        }

        if dp[amount] == i32::MAX {
            return -1;
        }

        dp[amount]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }
}
