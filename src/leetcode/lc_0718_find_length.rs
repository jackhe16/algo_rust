/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; nums2.len()]; nums1.len()];

        let mut result = 0;

        for (i, itemi) in nums1.iter().enumerate() {
            for (j, itemj) in nums2.iter().enumerate() {
                let p = if i < 1 || j < 1 { 0 } else { dp[i - 1][j - 1] };
                if itemi == itemj {
                    dp[i][j] = p + 1;
                }
                if dp[i][j] > result {
                    result = dp[i][j];
                }
            }
        }

        result
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]),
            5
        );
    }
}
