/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 */

struct Solution;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// @lc code=start

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut ans = vec![];

        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let mut temp = vec![];

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();
                temp.push(node.borrow().val);

                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
                }
            }

            ans.push(temp);
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
        assert_eq!(Solution::level_order(None), vec![] as Vec<Vec<i32>>);
    }
}
