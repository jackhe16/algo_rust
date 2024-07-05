/*
 * @lc app=leetcode.cn id=1035 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..len1 + 1 {
            for j in 1..len2 + 1 {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }

        dp[len1][len2]
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
}
