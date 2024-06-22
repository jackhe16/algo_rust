/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let mut dp = vec![vec![0; len]; len];

        for i in (0..len).rev() {
            for j in i..len {
                if s[i] == s[j] {
                    if j - i == 0 || j - i == 1 {
                        dp[i][j] = (j - i + 1) as i32;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1] + 2;
                    }
                } else {
                    if j < 1 {
                        dp[i][j] = dp[i + 1][j];
                    } else {
                        dp[i][j] = dp[i][j - 1].max(dp[i + 1][j]);
                    }
                }
            }
        }

        dp[0][len - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome_subseq(String::from("bbbab")),
            4
        );
    }
}
