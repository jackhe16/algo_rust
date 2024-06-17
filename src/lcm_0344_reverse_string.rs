/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            let tmp = s[l];
            s[l] = s[r];
            s[r] = tmp;
            l += 1;
            r -= 1;
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, ['o', 'l', 'l', 'e', 'h']);
    }
}
