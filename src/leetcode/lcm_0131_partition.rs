/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = vec![];
        let mut path = vec![];
        let chars = s.chars().collect::<Vec<_>>();

        Self::backtracking(&chars, &mut result, &mut path, 0);

        result
    }

    fn backtracking(
        chars: &Vec<char>,
        result: &mut Vec<Vec<String>>,
        path: &mut Vec<String>,
        start_index: usize,
    ) {
        if start_index >= chars.len() {
            result.push(path.to_vec());
            return;
        }

        for i in start_index..chars.len() {
            if Self::is_palindrome(chars, start_index, i) {
                let s = chars[start_index..i + 1].into_iter().collect::<String>();
                path.push(s);
                Self::backtracking(chars, result, path, i + 1);
                path.pop();
            }
        }
    }

    fn is_palindrome(chars: &Vec<char>, mut start: usize, mut end: usize) -> bool {
        while start < end {
            if chars[start] != chars[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::partition(String::from("aab")).len(), 2);
    }
}
