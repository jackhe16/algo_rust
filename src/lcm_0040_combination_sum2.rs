/*
 * @lc app=leetcode.cn id=40 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; candidates.len()];
        let mut candidates = candidates;
        candidates.sort();
        Self::backtracking(&mut result, &mut path, &candidates, target, 0, 0, &mut used);
        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        mut sum: i32,
        start_index: usize,
        used: &mut Vec<bool>,
    ) {
        if sum == target {
            result.push(path.to_vec());
            return;
        }

        for i in start_index..candidates.len() {
            if sum + candidates[i] <= target {
                if i > 0 && candidates[i] == candidates[i - 1] && used[i - 1] == false {
                    continue;
                }
                path.push(candidates[i]);
                used[i] = true;
                sum += candidates[i];
                Self::backtracking(result, path, candidates, target, sum, i + 1, used);
                sum -= candidates[i];
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
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8).len(),
            4
        );
    }
}
