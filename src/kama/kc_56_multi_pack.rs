pub struct Solution;

use std::cmp;

impl Solution {
    pub fn multi_pack(w: Vec<usize>, v: Vec<usize>, k: Vec<usize>, c: usize) -> usize {
        let mut dp = vec![0; c + 1];

        for i in 0..w.len() {
            for j in (w[i]..c + 1).rev() {
                let mut ki = 1;
                while ki <= k[i] && j >= ki * w[i] {
                    dp[j] = cmp::max(dp[j], dp[j - ki * w[i]] + v[i] * ki);
                    ki += 1;
                }
            }
        }

        dp[c]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::multi_pack(vec![1, 3, 4], vec![15, 20, 30], vec![2, 3, 2], 10),
            90
        )
    }
}
