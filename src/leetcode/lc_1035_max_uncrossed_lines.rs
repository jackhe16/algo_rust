/*
 * @lc app=leetcode.cn id=1035 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];

        for (i, chari) in nums1.iter().enumerate() {
            for (j, charj) in nums2.iter().enumerate() {
                let p = if i < 1 || j < 1 { 0 } else { dp[i - 1][j - 1] };
                if chari == charj {
                    dp[i][j] = p + 1;
                } else {
                    let p1 = if i < 1 { 0 } else { dp[i - 1][j] };
                    let p2 = if j < 1 { 0 } else { dp[i][j - 1] };
                    dp[i][j] = cmp::max(p1, p2);
                }
            }
        }

        dp[nums1.len() - 1][nums2.len() - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }
}
