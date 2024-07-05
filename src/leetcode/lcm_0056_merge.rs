/*
 * @lc app=leetcode.cn id=56 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        intervals.sort_by_key(|a| a[0]);

        ans.push(intervals[0].to_vec());
        for v in intervals.into_iter().skip(1) {
            let last = ans.last_mut().unwrap();
            if last[1] >= v[0] {
                last[1] = last[1].max(v[1]);
            } else {
                ans.push(v.to_vec());
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
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }
}
