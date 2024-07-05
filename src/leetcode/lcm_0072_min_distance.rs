/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = word2.chars().collect::<Vec<_>>();
        let len1 = word1.len();
        let len2 = word2.len();

        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for j in 0..len2 + 1 {
            dp[0][j] = j as i32;
        }
        for i in 0..len1 + 1 {
            dp[i][0] = i as i32;
        }

        for i in 1..len1 + 1 {
            for j in 1..len2 + 1 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]) + 1;
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
            Solution::min_distance(String::from("horse"), String::from("ros")),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_distance(String::from("intention"), String::from("execution")),
            5
        );
    }
}
