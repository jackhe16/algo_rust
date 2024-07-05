/*
 * @lc app=leetcode.cn id=232 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

pub struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while !self.stack_in.is_empty() {
                self.stack_out.push(self.stack_in.pop().unwrap());
            }
        }
        self.stack_out.pop().unwrap()
    }

    pub fn peek(&mut self) -> i32 {
        let res = self.pop();
        self.stack_out.push(res);
        res
    }

    pub fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MyQueue::new();
        obj.push(1);
        let ret_1 = obj.peek();
        let ret_2 = obj.pop();
        let ret_3 = obj.empty();
        assert_eq!(ret_1, 1);
        assert_eq!(ret_2, 1);
        assert_eq!(ret_3, true);
    }
}
