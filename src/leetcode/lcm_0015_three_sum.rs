/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut ans = Vec::new();

        for i in 0..n - 2 {
            let x = nums[i];
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if x + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }
            if x + nums[n - 2] + nums[n - 1] < 0 {
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let s = x + nums[j] + nums[k];
                if s > 0 {
                    k -= 1;
                } else if s < 0 {
                    j += 1;
                } else {
                    ans.push(vec![x, nums[j], nums[k]]);
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    k -= 1;
                    while k > j && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }
        }

        ans
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
    }
}
