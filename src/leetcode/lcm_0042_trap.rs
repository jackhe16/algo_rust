/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ans = 0;

        for i in 0..height.len() {
            let cur_val = height[i];

            while !stack.is_empty() {
                let mid_idx = *stack.last().unwrap();
                let mid_val = height[mid_idx];
                if cur_val >= mid_val {
                    stack.pop();
                    if !stack.is_empty() {
                        let left_idx = *stack.last().unwrap();
                        let left_val = height[left_idx];
                        let top = cur_val.min(left_val);
                        let area = (top - mid_val) * (i - left_idx - 1) as i32;
                        ans += area;
                    }
                } else {
                    break;
                }
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
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
