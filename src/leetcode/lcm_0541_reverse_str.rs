/*
 * @lc app=leetcode.cn id=541 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut c = 0;
        let k = k as usize;

        while 2 * k * c < n {
            let mut l = 2 * k * c;
            let mut r = cmp::min(2 * k * c + k - 1, n - 1);

            while l < r {
                let tmp = s[l];
                s[l] = s[r];
                s[r] = tmp;
                l += 1;
                r -= 1;
            }

            c += 1;
        }

        s.iter().collect::<String>()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reverse_str(String::from("abcdefg"), 2),
            String::from("bacdfeg")
        );
    }
}
