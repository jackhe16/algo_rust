/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut i = 0;
        let mut j = n - 1;
        let mut k = n;

        while i <= j {
            let si = nums[i] * nums[i];
            let sj = nums[j] * nums[j];
            if si < sj {
                result[k - 1] = sj;
                j -= 1;
            } else {
                result[k - 1] = si;
                i += 1;
            }
            k -= 1;
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
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            [0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            [4, 9, 9, 49, 121]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::sorted_squares(vec![1]), [1]);
    }
}
