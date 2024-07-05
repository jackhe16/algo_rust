/*
 * @lc app=leetcode.cn id=435 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by_key(|a| a[0]);

        let mut ans = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] < intervals[i - 1][1] {
                ans += 1;
                intervals[i][1] = intervals[i][1].min(intervals[i - 1][1]);
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
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
    }
}
