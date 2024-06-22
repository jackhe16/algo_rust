/*
 * @lc app=leetcode.cn id=337 lang=rust
 *
 */

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// @lc code=start

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (v1, v2) = Self::rob_tree(&root);

        cmp::max(v1, v2)
    }

    pub fn rob_tree(cur: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match cur {
            None => (0, 0),
            Some(node) => {
                let left = Self::rob_tree(&node.borrow_mut().left);
                let right = Self::rob_tree(&node.borrow_mut().right);

                (
                    cmp::max(left.0, left.1) + cmp::max(right.0, right.1),
                    node.borrow().val + left.0 + right.0,
                )
            }
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::rob(None), 0);
    }
}
