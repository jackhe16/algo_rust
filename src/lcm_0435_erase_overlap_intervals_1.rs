/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|a: &Vec<i32>| a[1]);

        let mut ans = 1;
        let mut end = intervals[0][1];
        for v in intervals.iter().skip(1) {
            if end <= v[0] {
                ans += 1;
                end = v[1];
            }
        }

        (intervals.len() - ans) as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }
}
