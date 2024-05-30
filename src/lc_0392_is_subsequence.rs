/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        if t.is_empty() {
            return false;
        }

        let mut dp = vec![vec![false; t.len()]; s.len()];

        for (i, chari) in s.chars().enumerate() {
            for (j, charj) in t.chars().enumerate() {
                if chari == charj {
                    let p = if i < 1 {
                        true
                    } else if j < 1 {
                        false
                    } else {
                        dp[i - 1][j - 1]
                    };
                    dp[i][j] = p;
                } else {
                    let p1 = if j < 1 { false } else { dp[i][j - 1] };
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
        assert!(Solution::is_subsequence(
            "abc".to_string(),
            "ahbgdc".to_string()
        ),);
    }

    #[test]
    fn test_2() {
        assert!(!Solution::is_subsequence(
            "axc".to_string(),
            "ahbgdc".to_string()
        ),);
    }

    #[test]
    fn test_3() {
        assert!(Solution::is_subsequence(
            "".to_string(),
            "ahbgdc".to_string()
        ),);
    }
}
