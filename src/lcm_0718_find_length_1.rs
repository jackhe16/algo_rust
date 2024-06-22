/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums2.len();
        let mut dp = vec![0; n];
        let mut ans = 0;

        for mi in nums1 {
            for j in (0..n).rev() {
                if mi == nums2[j] {
                    dp[j] = if j == 0 { 1 } else { dp[j - 1] + 1 };
                    ans = ans.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
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
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
            3
        );
    }
}
