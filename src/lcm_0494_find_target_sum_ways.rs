/*
 * @lc app=leetcode.cn id=494 lang=rust
 *
 */

struct Solution;

// @lc code=start

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as usize;
        let target = target.abs() as usize;
        if sum < target {
            return 0;
        }
        if (sum - target) % 2 == 1 {
            return 0;
        }

        let x = if sum == target {
            sum
        } else {
            (sum - target) / 2
        };
        let mut dp = vec![0; x + 1];
        dp[0] = 1;

        for n in nums {
            let n = n as usize;
            for j in (n..x + 1).rev() {
                dp[j] += dp[j - n];
            }
        }

        dp[x]
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 0], 1), 2);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0, 0, 0, 0, 1], 1),
            256
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::find_target_sum_ways(vec![0, 0, 0, 0, 0], 0), 32);
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::find_target_sum_ways(vec![7, 9, 3, 8, 0, 2, 4, 8, 3, 9], 0),
            0
        );
    }
}
