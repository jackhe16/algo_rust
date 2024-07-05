/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 总共max
        let mut dp0 = vec![1; nums.len()];
        // 含当前i的max
        let mut dp1 = vec![1; nums.len()];
        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp0[i] = dp0[i].max(dp1[j] + 1);
                    dp1[i] = dp1[i].max(dp1[j] + 1);
                } else {
                    dp0[i] = dp0[i].max(dp0[j]);
                }
            }
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
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::length_of_lis(vec![1, 2, 3, 0]), 3);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::length_of_lis(vec![4, 10, 4, 3, 8, 9]), 3);
    }
}
