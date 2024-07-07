//! # 后缀表达式
//!
//!

pub fn postfix_eval(expression: &str) -> i32 {
    let mut stack = vec![];

    for token in expression.split_whitespace() {
        match token.parse::<i32>() {
            Ok(num) => {
                stack.push(num);
            }
            Err(_) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match token {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => stack.push(a / b),
                    _ => panic!(),
                }
            }
        }
    }

    stack[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postfix_eval() {
        assert_eq!(postfix_eval("5 3 2 * +"), 11);
    }
}
