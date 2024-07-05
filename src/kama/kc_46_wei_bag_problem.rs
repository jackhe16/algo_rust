pub struct Solution;

use std::cmp;

impl Solution {
    pub fn wei_bag_problem(w: Vec<usize>, v: Vec<usize>, n: usize) -> usize {
        let len = w.len();
        let mut dp = vec![vec![0; n + 1]; len];

        for j in w[0]..n + 1 {
            dp[0][j] = v[0];
        }

        for i in 1..len {
            for j in 1..n + 1 {
                if j >= w[i] {
                    dp[i][j] = cmp::max(dp[i - 1][j], dp[i - 1][j - w[i]] + v[i]);
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
            Solution::wei_bag_problem(vec![2, 2, 3, 1, 5, 2], vec![2, 3, 1, 5, 4, 3], 1),
            5
        )
    }
}
