/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.chars().collect::<Vec<_>>();
        let mut stack = Vec::new();

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if stack.is_empty() || stack.pop().unwrap() != c {
                        return false;
                    }
                }
            }
            i += 1;
        }

        stack.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
