/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<_> = vec![];
        let mut path = vec![];
        Self::backtracking(&mut result, &mut path, n, k, 1);
        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        n: i32,
        k: i32,
        start_index: i32,
    ) {
        let len = path.len() as i32;

        if len == k {
            result.push(path.to_vec());
            return;
        }

        for i in start_index..=n - (k - len) + 1 {
            path.push(i);
            Self::backtracking(result, path, n, k, i + 1);
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
        assert_eq!(Solution::combine(4, 2).len(), 6);
    }
}
