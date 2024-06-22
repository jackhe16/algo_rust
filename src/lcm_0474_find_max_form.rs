/*
 * @lc app=leetcode.cn id=474 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in strs {
            let zero = s.chars().filter(|c| *c == '0').count();
            let one = s.chars().filter(|c| *c == '1').count();

            for i in (zero..m + 1).rev() {
                for j in (one..n + 1).rev() {
                    dp[i][j] = cmp::max(dp[i][j], dp[i - zero][j - one] + 1);
                }
            }
        }

        dp[m][n]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    String::from("10"),
                    String::from("0001"),
                    String::from("111001"),
                    String::from("1"),
                    String::from("0"),
                ],
                5,
                3
            ),
            4
        );
    }
}
