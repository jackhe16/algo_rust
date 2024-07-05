/*
 * @lc app=leetcode.cn id=491 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];

        Self::backtracking(&mut result, &mut path, &nums, 0);

        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_index: usize,
    ) {
        if path.len() > 1 {
            result.push(path.to_vec());
        }

        let mut set = HashSet::new();
        for i in start_index..nums.len() {
            if (!path.is_empty() && nums[i] < *path.last().unwrap()) || set.contains(&nums[i]) {
                continue;
            }
            set.insert(nums[i]);
            path.push(nums[i]);
            Self::backtracking(result, path, nums, i + 1);
            path.pop();
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_subsequences(vec![4, 6, 7, 7]).len(), 8);
    }
}
