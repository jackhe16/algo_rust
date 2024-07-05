/*
 * @lc app=leetcode.cn id=225 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

pub struct MyStack {
    queue: Vec<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack { queue: Vec::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        let n = self.queue.len();
        for _ in 0..n - 1 {
            let tmp = self.queue.remove(0);
            self.queue.push(tmp);
        }
        self.queue.remove(0)
    }

    pub fn top(&mut self) -> i32 {
        let res = self.pop();
        self.queue.push(res);
        res
    }

    pub fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut obj = MyStack::new();
        obj.push(1);
        obj.push(2);
        let ret_1 = obj.top();
        let ret_2 = obj.pop();
        let ret_3 = obj.empty();
        assert_eq!(ret_1, 2);
        assert_eq!(ret_2, 2);
        assert_eq!(ret_3, false);
    }
}
