/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len()]; text1.len()];

        for (i, chari) in text1.chars().enumerate() {
            for (j, charj) in text2.chars().enumerate() {
                let p = if i < 1 || j < 1 { 0 } else { dp[i - 1][j - 1] };
                if chari == charj {
                    dp[i][j] = p + 1;
                } else {
                    let p1 = if i < 1 { 0 } else { dp[i - 1][j] };
                    let p2 = if j < 1 { 0 } else { dp[i][j - 1] };
                    dp[i][j] = cmp::max(p1, p2);
                }
            }
        }

        dp[text1.len() - 1][text2.len() - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
