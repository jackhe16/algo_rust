/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start

impl Solution {
    #[allow(dead_code)]
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = dummy.as_mut();

        while let Some(mut node) = cur.next.take() {
            if let Some(mut next) = node.next.take() {
                node.next = next.next.take();
                next.next = Some(node);
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let head = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        });

        let result = Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        });

        assert_eq!(Solution::swap_pairs(Some(head)), Some(result));
    }
}
