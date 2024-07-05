/*
 * @lc app=leetcode.cn id=216 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();

        Self::backtracking(&mut result, &mut path, k, n, 1);

        result
    }

    fn backtracking(
        result: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        k: i32,
        n: i32,
        start_index: i32,
    ) {
        let len = path.len() as i32;
        let sum = path.iter().sum::<i32>();

        if sum > n {
            return;
        }

        if len == k {
            if sum == n {
                result.push(path.to_vec());
            }
            return;
        }

        for i in start_index..=9 - (k - len) + 1 {
            if sum + i - 1 >= n {
                return;
            }
            path.push(i);
            Self::backtracking(result, path, k, n, i + 1);
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
        assert_eq!(Solution::combination_sum3(3, 7).len(), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::combination_sum3(3, 9).len(), 3);
    }
}
