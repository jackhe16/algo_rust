/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.chars().collect::<Vec<_>>();
        let next = Self::get_next(&s);
        let n = s.len();

        if n == 0 {
            return false;
        }

        if next[n - 1] != 0 && n % (n - next[n - 1]) == 0 {
            return true;
        }

        false
    }

    pub fn get_next(s: &Vec<char>) -> Vec<usize> {
        let n = s.len();
        let mut j = 0;
        let mut next = vec![0; n];

        for i in 1..n {
            while j > 0 && s[i] != s[j] {
                j = next[j - 1];
            }
            if s[i] == s[j] {
                j += 1;
            }
            next[i] = j;
        }

        next
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abab")),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("aba")),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::repeated_substring_pattern(String::from("abcabcabcabc")),
            true
        );
    }
}
