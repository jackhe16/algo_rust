/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut stack = vec![];

        for v in nums2 {
            while !stack.is_empty() {
                let last = *stack.last().unwrap();
                if v > last {
                    stack.pop();
                    map.insert(last, v);
                } else {
                    break;
                }
            }

            stack.push(v);
        }

        nums1
            .iter()
            .map(|item| match map.get(item) {
                None => -1,
                Some(v) => *v,
            })
            .collect()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }
}
