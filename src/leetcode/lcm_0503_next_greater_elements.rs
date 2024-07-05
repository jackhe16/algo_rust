/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![-1; len];
        let mut stack = vec![];

        for i in 0..len * 2 {
            let v = nums[i % len];

            while !stack.is_empty() {
                let last = *stack.last().unwrap();
                if v > nums[last] {
                    stack.pop();
                    ans[last] = v;
                } else {
                    break;
                }
            }

            stack.push(i % len);
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
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
    }
}
