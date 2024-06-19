/*
 * @lc app=leetcode.cn id=452 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|a| a[0]);

        let mut c = 1;
        for i in 1..points.len() {
            if points[i][0] > points[i - 1][1] {
                c += 1;
            } else {
                points[i][1] = points[i][1].min(points[i - 1][1]);
            }
        }

        c
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_min_arrow_shots(
                vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12],]
            ),
            2
        );
    }
}
