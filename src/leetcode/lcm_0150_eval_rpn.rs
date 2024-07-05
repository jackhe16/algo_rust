/*
 * @lc app=leetcode.cn id=150 lang=rust
 *
 */

pub struct Solution;

// @lc code=start

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            let s = &tokens[i];

            match s.as_str() {
                "+" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
                "-" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a - b);
                }
                "*" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a * b);
                }
                "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a / b);
                }
                _ => stack.push(s.parse::<i32>().unwrap()),
            }

            i += 1;
        }

        stack.pop().unwrap()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::eval_rpn(vec![
                String::from("2"),
                String::from("1"),
                String::from("+"),
                String::from("3"),
                String::from("*")
            ]),
            9
        );
    }
}
