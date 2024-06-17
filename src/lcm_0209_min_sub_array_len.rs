/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = n + 1;
        let mut sum = 0; // 子数组元素和
        let mut left = 0; // 子数组左端点
        for (right, &x) in nums.iter().enumerate() {
            // 枚举子数组右端点
            sum += x;
            while sum >= target {
                // 满足要求
                ans = ans.min(right - left + 1);
                sum -= nums[left]; // 左端点右移
                left += 1;
            }
        }
        if ans <= n {
            ans as i32
        } else {
            0
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
}
