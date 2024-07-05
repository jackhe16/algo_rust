/*
 * @lc app=leetcode.cn id=860 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five = 0;
        let mut ten = 0;

        for bill in bills {
            match bill {
                5 => {
                    five += 1;
                }
                10 => {
                    if five <= 0 {
                        return false;
                    }
                    five -= 1;
                    ten += 1;
                }
                20 => {
                    if ten > 0 && five > 0 {
                        ten -= 1;
                        five -= 1;
                    } else if five >= 3 {
                        five -= 3;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }

        true
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
    }
}
