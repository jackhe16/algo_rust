/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 0;
        dp[1] = 0;

        for i in 2..n + 1 {
            dp[i] = cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2])
        }

        dp[n]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
