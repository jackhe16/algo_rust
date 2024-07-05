/*
 * @lc app=leetcode.cn id=135 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();

        let mut nums = vec![1; n];

        for i in 0..n - 1 {
            if ratings[i + 1] > ratings[i] {
                nums[i + 1] = nums[i] + 1;
            }
        }

        for i in (1..n).rev() {
            if ratings[i - 1] > ratings[i] {
                nums[i - 1] = nums[i - 1].max(nums[i] + 1);
            }
        }

        nums.iter().sum()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }
}
