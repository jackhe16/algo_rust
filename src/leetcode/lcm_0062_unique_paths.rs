/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![1; n]; m];

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }

        dp[m - 1][n - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
