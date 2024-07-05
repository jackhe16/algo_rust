/*
 * @lc app=leetcode.cn id=122 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];
            if diff > 0 {
                max += diff;
            }
        }

        max
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
