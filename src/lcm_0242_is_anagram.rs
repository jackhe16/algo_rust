/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut record = vec![0; 26];

        let baseChar = 'a';

        for byte in s.bytes() {
            record[byte as usize - baseChar as usize] += 1;
        }
        for byte in t.bytes() {
            record[byte as usize - baseChar as usize] -= 1;
        }

        record.iter().filter(|x| **x != 0).count() == 0
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );
    }
}
