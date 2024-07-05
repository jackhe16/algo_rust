/*
 * @lc app=leetcode.cn id=1005 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp::Reverse;

impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        nums.sort_by_key(|a| Reverse(a.abs()));

        for n in nums.iter_mut() {
            if *n < 0 && k > 0 {
                *n *= -1;
                k -= 1;
            }
        }

        if k % 2 == 1 {
            *nums.last_mut().unwrap() *= -1;
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
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
