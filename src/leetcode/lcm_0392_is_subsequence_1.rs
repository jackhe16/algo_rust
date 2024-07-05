/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut dp = vec![0; t.len() + 1];

        for (_i, si) in s.chars().enumerate() {
            let mut prev = 0;

            for (j, tj) in t.chars().enumerate() {
                let temp = dp[j + 1];

                if si == tj {
                    dp[j + 1] = prev + 1;
                } else {
                    dp[j + 1] = dp[j];
                }

                prev = temp;
            }
        }

        dp[t.len()] == s.len()
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
