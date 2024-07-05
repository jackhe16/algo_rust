/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

/// dp    0(r) 1(o) 2(s) 3
/// 0(h)  0    1    2    3
/// 1(o)  1    1    2    3
/// 2(r)  2    2    1    2
/// 3(s)  3    2    2    2
/// 4(e)  4    3    3    2
/// 5     5    4    4    3
///
/// horse -> ros
/// horse -> hors -> hos -> ros
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        #[allow(clippy::needless_range_loop)]
        for i in 0..word1.len() + 1 {
            dp[i][0] = i as i32;
        }
        for j in 0..word2.len() + 1 {
            dp[0][j] = j as i32;
        }

        for i in 1..word1.len() + 1 {
            for j in 1..word2.len() + 1 {
                if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = cmp::min(dp[i - 1][j - 1], cmp::min(dp[i - 1][j], dp[i][j - 1])) + 1;
                }
            }
        }

        dp[word1.len()][word2.len()]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
