/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect::<Vec<_>>();
        Self::remove_extra_spaces(&mut s);
        let n = s.len();
        Self::reverse(&mut s, 0, n - 1);

        let mut start = 0;
        for i in 0..=n {
            if i == n || s[i].is_ascii_whitespace() {
                Self::reverse(&mut s, start, i - 1);
                start = i + 1;
            }
        }

        s.iter().collect()
    }

    pub fn remove_extra_spaces(s: &mut Vec<char>) {
        let n = s.len();
        let mut i = 0;
        let mut slow = 0;
        while i < n {
            if !s[i].is_ascii_whitespace() {
                if slow != 0 {
                    s[slow] = ' ';
                    slow += 1;
                }

                while i < n && !s[i].is_ascii_whitespace() {
                    s[slow] = s[i];
                    slow += 1;
                    i += 1;
                }
            }

            i += 1;
        }
        s.truncate(slow);
    }

    pub fn reverse(s: &mut Vec<char>, mut begin: usize, mut end: usize) {
        while begin < end {
            let tmp = s[begin];
            s[begin] = s[end];
            s[end] = tmp;
            begin += 1;
            end -= 1;
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_words(String::from("the sky is blue")),
            String::from("blue is sky the")
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::reverse_words(String::from("  hello  world  ")),
            String::from("world hello")
        );
    }
}
