/*
 * @lc app=leetcode.cn id=39 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        Self::backtracking(&candidates, target, &mut result, &mut path, 0, 0);

        result
    }

    fn backtracking(
        candidates: &Vec<i32>,
        target: i32,
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        sum: i32,
        start_index: usize,
    ) {
        if sum == target {
            result.push(path.to_vec());
            return;
        }

        for i in start_index..candidates.len() {
            if sum + candidates[i] <= target {
                path.push(candidates[i]);
                Self::backtracking(candidates, target, result, path, sum + candidates[i], i);
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
        assert_eq!(Solution::combination_sum(vec![2, 3, 6, 7], 7).len(), 2);
    }
}
