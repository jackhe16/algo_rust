/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let len = s.len();
        let mut dp = vec![false; len + 1];
        dp[0] = true;

        for j in 1..len + 1 {
            for word in &word_dict {
                if j >= word.len() {
                    let right_eq = word.eq(&s[j - word.len()..j]);
                    let dp_left = dp[j - word.len()];
                    dp[j] = dp[j] || (dp_left && right_eq);
                }
            }
        }

        dp[len]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::word_break(
                String::from("leetcode"),
                vec![String::from("leet"), String::from("code")]
            ),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::word_break(
                String::from("applepenapple"),
                vec![String::from("apple"), String::from("pen")]
            ),
            true
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::word_break(
                String::from("catsandog"),
                vec![
                    String::from("cats"),
                    String::from("dog"),
                    String::from("sand"),
                    String::from("and"),
                    String::from("cat")
                ]
            ),
            false
        );
    }
}
