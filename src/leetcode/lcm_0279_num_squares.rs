/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        let mut i: usize = 1;
        while i * i <= n {
            for j in i * i..n + 1 {
                let j = j as usize;
                if dp[j - i * i] != i32::MAX {
                    dp[j] = cmp::min(dp[j], dp[j - i * i] + 1);
                }
            }

            i += 1;
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
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_squares(13), 2);
    }
}
