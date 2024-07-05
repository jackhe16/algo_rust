/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn get_next(n: i32) -> i32 {
            let mut n = n;
            let mut sum = 0;
            while n > 0 {
                sum += (n % 10) * (n % 10);
                n /= 10;
            }
            sum
        }

        let mut n = n;
        let mut cycle_set = HashSet::new();
        while n != 1 && !cycle_set.contains(&n) {
            cycle_set.insert(n);
            n = get_next(n);
        }

        n == 1
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_happy(19), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_happy(2), false);
    }
}
