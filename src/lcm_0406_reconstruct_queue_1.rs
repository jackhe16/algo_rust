/*
 * @lc app=leetcode.cn id=406 lang=rust
 *
 */

struct Solution;

// @lc code=start

use std::collections::LinkedList;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = LinkedList::new();

        people.sort_by(|a, b| {
            if a[0] == b[0] {
                return a[1].cmp(&b[1]);
            }
            b[0].cmp(&a[0])
        });

        queue.push_back(people[0].to_vec());
        for item in people.iter().skip(1) {
            let mut rqueue = queue.split_off(item[1] as usize);
            queue.push_back(item.to_vec());
            queue.append(&mut rqueue);
        }

        queue.into_iter().collect()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
    }
}
