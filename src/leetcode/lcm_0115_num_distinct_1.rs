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
        let mut dp = vec![0; len_s + 1];

        for i in 1..len_t + 1 {
            let mut prev = 0;

            for j in 1..len_s + 1 {
                let temp = dp[j];

                if t[i - 1] == s[j - 1] {
                    if i == 1 {
                        dp[j] = 1 + dp[j - 1];
                    } else {
                        dp[j] = prev + dp[j - 1]
                    };
                } else {
                    dp[j] = dp[j - 1];
                }

                prev = temp;
            }
        }

        dp[len_s]
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

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_distinct(String::from("babgbag"), String::from("bag")),
            5
        );
    }
}
