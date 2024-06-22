struct Solution;

impl Solution {
    pub fn climb_stairs(n: usize, m: usize) -> usize {
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for j in 1..n + 1 {
            for i in 1..m + 1 {
                if j >= i {
                    dp[j] += dp[j - i];
                }
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
        assert_eq!(Solution::climb_stairs(3, 2), 3)
    }
}
