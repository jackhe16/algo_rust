/*
 * @lc app=leetcode.cn id=78 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
        result.push(path.to_vec());

        for i in start_index..nums.len() {
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
        assert_eq!(Solution::subsets(vec![1, 2, 3]).len(), 8);
    }
}
