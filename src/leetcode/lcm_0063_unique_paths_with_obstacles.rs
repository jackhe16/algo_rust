/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            if obstacle_grid[i][0] == 0 {
                dp[i][0] = 1;
            } else {
                break;
            }
        }
        for j in 0..n {
            if obstacle_grid[0][j] == 0 {
                dp[0][j] = 1;
            } else {
                break;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 0 {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
    }
}
