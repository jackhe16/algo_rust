/*
 * @lc app=leetcode.cn id=199 lang=rust
 *
 */

pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// @lc code=start

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut queue = VecDeque::new();

        if root.is_some() {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let right = queue.back().unwrap().as_ref().unwrap();
            ans.push(right.borrow().val);

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap().unwrap();

                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone());
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
        let tree = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        }));

        assert_eq!(Solution::right_side_view(Some(tree)), vec![1, 2]);
    }
}
