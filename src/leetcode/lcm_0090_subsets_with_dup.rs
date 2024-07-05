/*
 * @lc app=leetcode.cn id=90 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];
        let mut used = vec![false; nums.len()];
        let mut nums = nums;
        nums.sort();

        Self::backtracking(&mut result, &mut path, &nums, 0, &mut used);

        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        nums: &Vec<i32>,
        start_index: usize,
        used: &mut Vec<bool>,
    ) {
        result.push(path.to_vec());

        for i in start_index..nums.len() {
            if i > 0 && nums[i - 1] == nums[i] && used[i - 1] == false {
                continue;
            }
            path.push(nums[i]);
            used[i] = true;
            Self::backtracking(result, path, nums, i + 1, used);
            used[i] = false;
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
        assert_eq!(Solution::subsets_with_dup(vec![1, 2, 2]).len(), 6);
    }
}
