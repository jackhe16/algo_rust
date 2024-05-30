/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    #[allow(dead_code)]
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        // 总max
        let mut dp0 = vec![1; nums.len()];
        // 含i的max
        let mut dp1 = vec![1; nums.len()];
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                dp1[i] = dp1[i - 1] + 1;
            } else {
                dp1[i] = 1;
            }
            dp0[i] = cmp::max(dp0[i - 1], dp1[i]);
        }
        dp0[nums.len() - 1]
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

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
