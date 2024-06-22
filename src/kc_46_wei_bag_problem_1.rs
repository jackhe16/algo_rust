struct Solution;

use std::cmp;

impl Solution {
    pub fn wei_bag_problem(w: Vec<usize>, v: Vec<usize>, n: usize) -> usize {
        let len = w.len();
        let mut dp = vec![0; n + 1];

        for i in 0..len {
            for j in (w[i]..n + 1).rev() {
                dp[j] = cmp::max(dp[j], dp[j - w[i]] + v[i]);
            }
        }

        dp[n]
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

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::wei_bag_problem(vec![1, 3, 4], vec![15, 20, 30], 4),
            35
        )
    }
}
