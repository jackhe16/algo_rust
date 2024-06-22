/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        let len1 = text1.len();
        let len2 = text2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..len1 + 1 {
            for j in 1..len2 + 1 {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }

        dp[len1][len2]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_subsequence(String::from("abcde"), String::from("ace")),
            3
        );
    }
}
