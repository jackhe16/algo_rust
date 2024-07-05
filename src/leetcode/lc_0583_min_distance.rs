/*
 * @lc app=leetcode.cn id=583 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::cmp;

impl Solution {
    //
    // pub fn min_distance(word1: String, word2: String) -> i32 {
    //     let mut dp = vec![vec![0; word2.len()]; word1.len()];

    //     if word1.is_empty() {
    //         return word2.len() as i32;
    //     }
    //     if word2.is_empty() {
    //         return word1.len() as i32;
    //     }

    //     for (i, chari) in word1.chars().enumerate() {
    //         for (j, charj) in word2.chars().enumerate() {
    //             if chari == charj {
    //                 let p = if i < 1 && j < 1 {
    //                     0
    //                 } else if i < 1 {
    //                     (j) as i32
    //                 } else if j < 1 {
    //                     (i) as i32
    //                 } else {
    //                     dp[i - 1][j - 1]
    //                 };
    //                 dp[i][j] = p;
    //             } else {
    //                 let p1 = if i < 1 { (j + 1) as i32 } else { dp[i - 1][j] };
    //                 let p2 = if j < 1 { (i + 1) as i32 } else { dp[i][j - 1] };
    //                 dp[i][j] = cmp::min(p1 + 1, p2 + 1);
    //             }
    //         }
    //     }

    //     dp[word1.len() - 1][word2.len() - 1]
    // }

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
                    dp[i][j] = cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1);
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
            Solution::min_distance("sea".to_string(), "eat".to_string()),
            2
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_distance("leetcode".to_string(), "etco".to_string()),
            4
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::min_distance("".to_string(), "etco".to_string()),
            4
        );
    }
}
