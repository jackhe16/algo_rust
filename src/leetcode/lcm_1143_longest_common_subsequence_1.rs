/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        let len2 = text2.len();
        let mut dp = vec![0; len2 + 1];

        for c1 in text1 {
            let mut prev = 0;

            for j in 1..len2 + 1 {
                let temp = dp[j];

                if text2[j - 1] == c1 {
                    dp[j] = prev + 1;
                } else {
                    dp[j] = dp[j].max(dp[j - 1]);
                }

                prev = temp;
            }
        }

        dp[len2]
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
