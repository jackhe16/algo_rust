/*
 * @lc app=leetcode.cn id=377 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;

        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for j in 1..target + 1 {
            for &n in &nums {
                let n = n as usize;
                if j >= n {
                    dp[j] += dp[j - n];
                }
            }
        }

        dp[target]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 3), 4);
    }
}
