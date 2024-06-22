/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![1; len];

        let mut ans = 1;

        for i in 1..len {
            if nums[i] > nums[i - 1] {
                dp[i] = dp[i - 1] + 1;
                ans = ans.max(dp[i]);
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
        assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }
}
