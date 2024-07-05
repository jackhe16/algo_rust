/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; cmp::max(2, n + 1)];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..n + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }

    //
    // pub fn fib(n: i32) -> i32 {
    //     let n = n as usize;
    //     let mut dp = [0, 1];
    //     for _ in 2..n + 1 {
    //         let dpi = dp[1] + dp[0];
    //         dp[0] = dp[1];
    //         dp[1] = dpi;
    //     }
    //     if n < 2 {
    //         dp[n]
    //     } else {
    //         dp[1]
    //     }
    // }

    //
    // pub fn fib(n: i32) -> i32 {
    //     (0..n).fold((0, 1), |t, _| (t.1, t.1 + t.0)).0
    // }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::fib(4), 3);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::fib(0), 0);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::fib(1), 1);
    }
}
