/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];

        Self::backtracking(&nums, &mut result, &mut path, &mut used);

        result
    }

    fn backtracking(
        nums: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        used: &mut Vec<bool>,
    ) {
        if path.len() == nums.len() {
            result.push(path.to_vec());
            return;
        }

        for i in 0..nums.len() {
            if used[i] == false {
                path.push(nums[i]);
                used[i] = true;
                Self::backtracking(nums, result, path, used);
                used[i] = false;
                path.pop();
            }
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::permute(vec![1, 2, 3]).len(), 6);
    }
}
