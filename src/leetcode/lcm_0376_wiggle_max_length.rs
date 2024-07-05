/*
 * @lc app=leetcode.cn id=376 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut pre = 0;
        for i in 0..nums.len() - 1 {
            let cur = nums[i + 1] - nums[i];
            if pre >= 0 && cur < 0 || pre <= 0 && cur > 0 {
                count += 1;
                pre = cur;
            }
        }

        count
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
    }
}
