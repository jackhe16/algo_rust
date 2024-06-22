struct Solution;

use std::cmp;

impl Solution {
    pub fn complete_pack(w: Vec<usize>, v: Vec<usize>, n: usize) -> usize {
        let len = w.len();
        let mut dp = vec![0; n + 1];

        for i in 0..len {
            for j in w[i]..n + 1 {
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
