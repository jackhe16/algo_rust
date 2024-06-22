/*
 * @lc app=leetcode.cn id=1049 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>();
        let target = sum as usize / 2;
        let mut dp = vec![0; target + 1];

        for n in stones {
            for j in (n as usize..target + 1).rev() {
                dp[j] = cmp::max(dp[j], dp[j - n as usize] + n);
            }
        }

        (sum - dp[target]) - dp[target]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
    }
}
