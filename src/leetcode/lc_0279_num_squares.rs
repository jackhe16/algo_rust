/*
 * @lc app=leetcode.cn id=279 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let un = n as usize;
        let mut dp: Vec<i32> = vec![0; un + 1];
        for i in 1..un + 1 {
            dp[i] = i as i32;

            let mut j = 1;
            while j * j <= i {
                dp[i] = cmp::min(dp[i], dp[i - j * j] + 1);

                j += 1;
            }
        }
        dp[un]
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
