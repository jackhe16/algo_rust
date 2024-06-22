/*
 * @lc app=leetcode.cn id=647 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let mut dp = vec![vec![false; len]; len];

        let mut ans = 0;

        for i in (0..len).rev() {
            for j in i..len {
                if s[i] == s[j] {
                    if j - i == 0 || j - i == 1 || dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_substrings(String::from("abc")), 3);
    }
}
