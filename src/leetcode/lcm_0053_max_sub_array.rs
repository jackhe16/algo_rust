/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;

        let mut acc = 0;
        for n in nums {
            acc += n;
            max = max.max(acc);
            acc = acc.max(0);
        }

        max
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
