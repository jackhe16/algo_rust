/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut l = 0;
        let mut r = n as usize - 1;
        let mut t = 0;
        let mut b = n as usize - 1;

        let mut mat = vec![vec![0; n as usize]; n as usize];
        let mut num = 1;

        while l < r || t < b {
            for i in l..=r {
                mat[t][i] = num;
                num += 1;
            }
            t += 1;

            for i in t..=b {
                mat[i][r] = num;
                num += 1;
            }
            r -= 1;

            for i in (l..=r).rev() {
                mat[b][i] = num;
                num += 1;
            }
            b -= 1;

            for i in (t..=b).rev() {
                mat[i][l] = num;
                num += 1;
            }
            l += 1;
        }

        if n & 1 == 1 {
            mat[l][r] = num;
        }

        mat
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::generate_matrix(3),
            [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::generate_matrix(2), [[1, 2], [4, 3]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::generate_matrix(5),
            [
                [1, 2, 3, 4, 5],
                [16, 17, 18, 19, 6],
                [15, 24, 25, 20, 7],
                [14, 23, 22, 21, 8],
                [13, 12, 11, 10, 9]
            ]
        );
    }
}
