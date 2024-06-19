/*
 * @lc app=leetcode.cn id=455 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut l = 0;
        let mut r = 0;
        let len_g = g.len();
        let len_s = s.len();

        while l < len_g && r < len_s {
            if s[r] >= g[l] {
                l += 1;
            }
            r += 1;
        }

        l as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]),
            2
        );
    }
}
