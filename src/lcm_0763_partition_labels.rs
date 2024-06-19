/*
 * @lc app=leetcode.cn id=763 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut fars = vec![0; 26];

        for (i, &c) in s.as_bytes().iter().enumerate() {
            fars[(c - b'a') as usize] = i;
        }

        let mut ans = vec![];
        let mut l = 0;
        let mut r = 0;

        for (i, &c) in s.as_bytes().iter().enumerate() {
            r = r.max(fars[(c - b'a') as usize]);
            if i == r {
                ans.push((r - l + 1) as i32);
                l = i + 1;
            }
        }

        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::partition_labels(String::from("ababcbacadefegdehijhklij")),
            vec![9, 7, 8]
        );
    }
}
