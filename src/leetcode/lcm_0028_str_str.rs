/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 *
 */

// 如何更好地理解和掌握 KMP 算法? - 灵茶山艾府的回答 - 知乎 https://www.zhihu.com/question/21923021/answer/37475572

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();
        let n = haystack.len();
        let m = needle.len();
        let mut next = vec![0; m];

        let mut j = 0;
        for i in 1..m {
            // 回退遍历所有子匹配
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            next[i] = j;
        }

        let mut j = 0;
        for i in 0..n {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == m {
                return (i - m + 1) as i32;
            }
        }

        -1
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::str_str(String::from("aabaabaaf"), String::from("aabaaf")),
            3
        );
    }
}
