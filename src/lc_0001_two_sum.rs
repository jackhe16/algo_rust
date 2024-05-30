/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();

        for (i, &item) in nums.iter().enumerate() {
            if let Some(&v) = m.get(&(target - item)) {
                return vec![v, i as i32];
            }
            m.insert(item, i as i32);
        }

        vec![]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
