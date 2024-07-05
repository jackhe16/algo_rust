/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let digits = digits.chars().collect::<Vec<_>>();

        let mut result = Vec::new();
        let mut path = Vec::new();

        Self::backtracking(&mut result, &mut path, &digits, 0, &map);

        result
    }

    fn backtracking(
        result: &mut Vec<String>,
        path: &mut Vec<char>,
        digits: &Vec<char>,
        deep: usize,
        map: &HashMap<char, Vec<char>>,
    ) {
        if deep == digits.len() {
            result.push(path.to_vec().iter().collect());
            return;
        }

        let digit = digits[deep];
        let chars = map.get(&digit).unwrap();

        for &char in chars {
            path.push(char);
            Self::backtracking(result, path, &digits, deep + 1, map);
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
        assert_eq!(Solution::letter_combinations(String::from("23")).len(), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::letter_combinations(String::from("")).len(), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::letter_combinations(String::from("2")).len(), 3);
    }
}
