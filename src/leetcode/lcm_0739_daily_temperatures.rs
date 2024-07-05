/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![0; temperatures.len()];

        for (i, &v) in temperatures.iter().enumerate() {
            while !stack.is_empty() && v > temperatures[*stack.last().unwrap()] {
                let stack_i = stack.pop().unwrap();
                ans[stack_i] = (i - stack_i) as i32;
            }
            stack.push(i);
        }

        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            [1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
