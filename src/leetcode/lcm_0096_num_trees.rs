/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let len = n as usize;
        let mut dp = vec![0; len + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..len + 1 {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - j - 1];
            }
        }

        dp[len]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
}
