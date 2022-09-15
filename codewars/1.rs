/// Here is an implementation of https://www.codewars.com/kata/577e9095d648a15b800000d4/discuss

use std::collections::VecDeque;

enum Op {
    Substract,
    Add,
    Multiply,
    Divide
}

enum ExprNode {
    Value(i32),
    Operator(Op)
}

impl ExprNode {
    fn from_str(s: &str) -> Self {
        if let Ok(n) = s.parse::<i32>() {
            ExprNode::Value(n)
        }
        else {
            match s {
                "-" => ExprNode::Operator(Op::Substract),
                "+" => ExprNode::Operator(Op::Add),
                "*" => ExprNode::Operator(Op::Multiply),
                "/" => ExprNode::Operator(Op::Divide),
                _ => panic!("not supported yet")
            }
        }
    }
}

fn postfix_evaluator(expr: &str) -> i64 {
    let mut stack = VecDeque::from([]);

    for node  in expr.split_whitespace().map(|s| ExprNode::from_str(s)){
        match node {
            ExprNode::Value(v) => stack.push_back(v as i64),
            ExprNode::Operator(op) => {
                let a = stack.pop_back().unwrap();
                let b = stack.pop_back().unwrap();
                match op {
                    Op::Substract => {
                        stack.push_back(a - b);
                    },
                    Op::Add => {
                        stack.push_back(a + b);
                    },
                    Op::Multiply => {
                        stack.push_back(a * b);
                    },
                    Op::Divide => {
                        stack.push_back(b / a);
                    },
                }
            },
        }
    }
    stack.pop_back().unwrap()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Simple addition
        assert_eq!(postfix_evaluator("2 3 +"), 5);

        // Addition with negative numbers
        assert_eq!(postfix_evaluator("2 -3 +"), -1);

        // Constant numbers
        assert_eq!(postfix_evaluator("1"), 1);
        assert_eq!(postfix_evaluator("-1"), -1);

        // Complex expressions
        assert_eq!(postfix_evaluator("2 3 9 4 / + *"), 10);
        assert_eq!(postfix_evaluator("3 4 9 / *"), 0);
        assert_eq!(postfix_evaluator("4 8 + 6 5 - * 3 2 - 2 2 + * /"), 3);

        // Multi-digit
        assert_eq!(postfix_evaluator("21 21 +"), 42);
    }
}
