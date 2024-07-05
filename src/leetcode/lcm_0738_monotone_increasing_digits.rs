/*
 * @lc app=leetcode.cn id=738 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut n_bytes = n.to_string().into_bytes();

        let mut flag = n_bytes.len();
        for i in (1..n_bytes.len()).rev() {
            if n_bytes[i - 1] > n_bytes[i] {
                flag = i;
                n_bytes[i - 1] -= 1;
            }
        }

        for v in n_bytes.iter_mut().skip(flag) {
            *v = 57;
        }

        // n_bytes
        //     .iter()
        //     .map(|b| char::from_u32((*b).into()).unwrap())
        //     .collect::<String>()
        //     .parse::<i32>()
        //     .unwrap()

        n_bytes
            .into_iter()
            .fold(0, |acc, v| acc * 10 + v as i32 - 48)
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::monotone_increasing_digits(10), 9);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::monotone_increasing_digits(332), 299);
    }
}
