/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cfar = 0;
        let mut nfar = 0;
        let mut count = 0;

        for (i, &n) in nums.iter().enumerate().take(nums.len() - 1) {
            nfar = nfar.max(n as usize + i);
            if i == cfar {
                cfar = nfar;
                count += 1;
            }
        }

        count
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
