/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

        for (i, si) in s.chars().enumerate() {
            for (j, tj) in t.chars().enumerate() {
                if si == tj {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j];
                }
            }
        }

        dp[s.len()][t.len()] == s.len()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
    }
}
