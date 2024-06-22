/*
 * @lc app=leetcode.cn id=518 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;

        for coin in coins {
            let coin = coin as usize;
            for j in coin..amount + 1 {
                dp[j] += dp[j - coin];
            }
        }

        dp[amount] as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::change(5, vec![1, 2, 5],), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::change(5, vec![2, 3, 5],), 2);
    }
}
