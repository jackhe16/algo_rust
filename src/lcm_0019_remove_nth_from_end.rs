/*
 * @lc app=leetcode.cn id=19 lang=rust
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut count = n;
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut fast = &dummy.clone();
        let mut slow = &mut dummy;

        while count > 0 {
            fast = fast.next.as_ref().unwrap();
            count -= 1;
        }

        while fast.next.is_some() {
            fast = fast.next.as_ref().unwrap();
            slow = slow.next.as_mut().unwrap();
        }

        slow.next = slow.next.as_mut().unwrap().next.take();

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
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        });

        let result = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        });

        assert_eq!(Solution::remove_nth_from_end(Some(head), 2), Some(result));
    }
}
