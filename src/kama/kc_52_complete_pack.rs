pub struct Solution;

use std::cmp;

impl Solution {
    pub fn complete_pack(w: Vec<usize>, v: Vec<usize>, n: usize) -> usize {
        let len = w.len();
        let mut dp = vec![vec![0; n + 1]; len];

        for j in w[0]..n + 1 {
            dp[0][j] = dp[0][j - w[0]] + v[0];
        }

        for i in 1..len {
            for j in 1..n + 1 {
                if j >= w[i] {
                    dp[i][j] = cmp::max(dp[i - 1][j], dp[i][j - w[i]] + v[i]);
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }

        dp[len - 1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::complete_pack(vec![1, 2, 3, 4], vec![2, 4, 4, 5], 5),
            10
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::complete_pack(vec![3, 2, 1], vec![10, 5, 10], 6),
            60
        )
    }
}
