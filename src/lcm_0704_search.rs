/*
 * @lc app=leetcode.cn id=704 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] > target {
                // 避免溢出
                // mid==0说明left == right == 0, 且由第一个if条件nums[0] !== target, 返回-1
                if mid == 0 {
                    return -1;
                }
                right = mid - 1;
            } else if nums[mid] < target {
                left = mid + 1;
            }
        }

        -1
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::search(vec![2, 5], 0), -1);
    }
}
