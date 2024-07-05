/*
 * @lc app=leetcode.cn id=115 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let t = t.chars().collect::<Vec<_>>();
        let len_s = s.len();
        let len_t = t.len();
        let mut dp = vec![vec![0; len_s + 1]; len_t + 1];

        for j in 0..len_s + 1 {
            dp[0][j] = 1;
        }

        for i in 1..len_t + 1 {
            for j in i..len_s + 1 {
                if t[i - 1] == s[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }

        dp[len_t][len_s]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")),
            3
        );
    }
}
