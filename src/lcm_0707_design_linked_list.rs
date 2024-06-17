/*
 * @lc app=leetcode.cn id=707 lang=rust
 *
 */

#[allow(dead_code)]
struct Solution;

// @lc code=start

#[derive(Debug)]
struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { val: 0, next: None }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 {
            return -1;
        }

        let mut cur = &self.next;
        let mut i = 0;

        while let Some(node) = cur {
            if i == index {
                return node.val;
            }

            i += 1;
            cur = &node.next;
        }

        -1
    }

    fn add_at_head(&mut self, val: i32) {
        self.next = Some(Box::new(MyLinkedList {
            val: val,
            next: self.next.take(),
        }))
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(MyLinkedList { val, next: None });
        let mut last_node = &mut self.next;
        while let Some(node) = last_node {
            last_node = &mut node.next;
        }
        *last_node = Some(new_node);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
        } else {
            let mut cur = &mut self.next;
            let mut i = 1;
            while let Some(node) = cur {
                if i == index {
                    let new_node = Box::new(MyLinkedList {
                        val,
                        next: node.next.take(),
                    });
                    node.next = Some(new_node);
                    break;
                }
                i += 1;
                cur = &mut node.next;
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 {
            return;
        }

        let mut cur = self;
        let mut i = 0;

        while let Some(node) = cur.next.take() {
            if i == index {
                cur.next = node.next;
                break;
            }
            i += 1;
            cur.next = Some(node);
            cur = cur.next.as_mut().unwrap();
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let index = 1;
        let val = 0;
        let mut obj = MyLinkedList::new();
        let ret_1: i32 = obj.get(index);
        obj.add_at_head(val);
        obj.add_at_tail(val);
        obj.add_at_index(index, val);
        obj.delete_at_index(index);
    }
}
