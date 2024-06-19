/*
 * @lc app=leetcode.cn id=93 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    #[allow(dead_code)]
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = vec![];

        let len = s.len();
        if len < 4 || len > 12 {
            return result;
        }

        let mut chars = s.chars().collect::<Vec<_>>();

        Self::backtracking(&mut chars, &mut result, 0, 0);

        result
    }

    fn backtracking(
        chars: &mut Vec<char>,
        result: &mut Vec<String>,
        start_index: usize,
        mut points: usize,
    ) {
        let len = chars.len();

        if points == 3 {
            if Self::is_valid(chars, start_index, len - 1) {
                result.push(chars.iter().collect::<String>());
            }
            return;
        }

        let right = cmp::min(len, start_index + 3);
        for i in start_index..right {
            if Self::is_valid(chars, start_index, i) {
                chars.insert(i + 1, '.');
                points += 1;
                Self::backtracking(chars, result, i + 2, points);
                points -= 1;
                chars.remove(i + 1);
            } else {
                break;
            }
        }
    }

    fn is_valid(chars: &Vec<char>, start: usize, end: usize) -> bool {
        if start > end {
            return false;
        }

        if chars[start] == '0' && end > start {
            return false;
        }

        let s = chars[start..end + 1].iter().collect::<String>();
        let n = s.parse::<i32>().unwrap();

        if n > 255 {
            return false;
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
        assert_eq!(
            Solution::restore_ip_addresses(String::from("25525511135")).len(),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("101023")),
            [
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
