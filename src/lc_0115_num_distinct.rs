/*
 * @lc app=leetcode.cn id=115 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

/// s: babgbag
/// t: bag
/// dp[i][j]: 在 s[0..i + 1] 的子序列中 t[0..j + 1] 出现的个数
/// dp[i][j] = if s[i] == t[j] {
///     dp[i - 1][j - 1] + dp[i - 1][j]
/// } else {
///     dp[i - 1][j]
/// }
/// dp[i][j] = if j < 0 { 1 } else if i < 0 { 0 } else { dp[i][j] };
/// dp 0 1 2
/// 0  1 0 0
/// 1  1 1 0
/// 2  2 1 0
/// 3  2 1 1
/// 4  3 1 1
/// 5  3 4 1
/// 6  3 4 5
/// dp[s.len() - 1][t.len() - 1]
impl Solution {
    #[allow(dead_code)]
    pub fn num_distinct(s: String, t: String) -> i32 {
        if t.is_empty() {
            return 1;
        }
        if s.is_empty() {
            return 0;
        }

        let mut dp = vec![vec![0; t.len()]; s.len()];

        for (i, chari) in s.chars().enumerate() {
            for (j, charj) in t.chars().enumerate() {
                if chari == charj {
                    let p = if j < 1 {
                        1
                    } else if i < 1 {
                        0
                    } else {
                        dp[i - 1][j - 1]
                    };
                    let p1 = if i < 1 { 0 } else { dp[i - 1][j] };
                    dp[i][j] = p + p1;
                } else {
                    let p1 = if i < 1 { 0 } else { dp[i - 1][j] };
                    dp[i][j] = p1;
                }
            }
        }

        dp[s.len() - 1][t.len() - 1]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }
}
