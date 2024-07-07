//! # 中缀转后缀
//!
//!

pub fn infix_to_postfix(infix: &str) -> String {
    let mut postfix = vec![];
    let mut stack = vec![];

    let precedence = |op| match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    };

    for token in infix.chars() {
        match token {
            c if token.is_alphanumeric() => postfix.push(c),
            '(' => stack.push('('),
            ')' => {
                while let Some(top) = stack.pop() {
                    if top == '(' {
                        break;
                    }
                    postfix.push(top);
                }
            }
            '+' | '-' | '*' | '/' => {
                while let Some(top) = stack.last() {
                    if *top == '(' || precedence(*top) < precedence(token) {
                        break;
                    }
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
            _ => {}
        }
    }

    while let Some(top) = stack.pop() {
        if top == '(' {
            panic!()
        }
        postfix.push(top);
    }

    postfix
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        assert_eq!(infix_to_postfix("5 * (3 + 2)"), "5 3 2 + *".to_string());
    }
}
