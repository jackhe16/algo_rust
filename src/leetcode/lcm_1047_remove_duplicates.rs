/*
 * @lc app=leetcode.cn id=1047 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        let chars = s.chars().collect::<Vec<_>>();

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];

            if !stack.is_empty() && stack[stack.len() - 1] == c {
                stack.pop();
            } else {
                stack.push(c);
            }

            i += 1;
        }

        stack.iter().collect()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::remove_duplicates(String::from("abbaca")),
            String::from("ca")
        );
    }
}
