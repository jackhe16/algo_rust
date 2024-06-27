/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        heights.insert(0, 0);
        heights.push(0);

        let mut ans = 0;
        let mut stack = vec![];
        stack.push(0);

        for i in 1..heights.len() {
            while !stack.is_empty() {
                let last = *stack.last().unwrap();
                if heights[i] < heights[last] {
                    stack.pop();
                    let left_idx = *stack.last().unwrap();

                    let h = heights[last];
                    let w = (i - left_idx - 1) as i32;
                    let area = w * h;

                    ans = ans.max(area);
                } else {
                    if heights[i] == heights[last] {
                        stack.pop();
                    }

                    stack.push(i);
                    break;
                }
            }
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
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_rectangle_area(vec![0, 9]), 9);
    }
}
