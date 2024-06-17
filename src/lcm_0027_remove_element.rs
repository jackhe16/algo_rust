/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        let mut fast = 0;

        while fast < nums.len() {
            if nums[fast] != val {
                nums[k] = nums[fast];
                k = k + 1;
            }
            fast = fast + 1;
        }

        return k as i32;
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        assert_eq!(nums, vec![2, 2, 2, 3])
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
        assert_eq!(nums, vec![0, 1, 3, 0, 4, 0, 4, 2])
    }
}
