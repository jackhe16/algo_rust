/*
 * @lc app=leetcode.cn id=134 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut acc = 0;
        let mut ans = 0;
        for i in 0..gas.len() {
            cur += gas[i] - cost[i];
            acc += gas[i] - cost[i];
            if cur < 0 {
                ans = i + 1;
                cur = 0;
            }
        }

        if acc < 0 {
            return -1;
        }

        ans as i32
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]),
            4,
        );
    }
}
