/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len];
        dp[0] = nums[0];
        let mut ans = nums[0];

        for i in 1..len {
            dp[i] = (dp[i - 1] + nums[i]).max(nums[i]);
            ans = ans.max(dp[i]);
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
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
