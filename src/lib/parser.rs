use std::collections::VecDeque;

use lib::lexer::{Operator, tokenize, Token, Number};
use std::fmt;

pub enum Value {
    Number(Number),
    Operator(Operator),
    Nothing
}

pub struct ASTNode {
    value: Value,
    lhs: Option<Box<ASTNode>>,
    rhs: Option<Box<ASTNode>>,
}

type EvaluationResult = f64;

pub trait Evaluate {
    fn eval(self) -> EvaluationResult;
}

pub fn eval_node(operator: Operator, lhs_val: ASTNode, rhs_val: ASTNode) -> EvaluationResult {
    let lhs = lhs_val.eval();
    let rhs = rhs_val.eval();

    match operator {
        Operator::Add => lhs + rhs,
        Operator::Substract => lhs - rhs,
        Operator::Multiply => lhs * rhs,
        Operator::Divide => lhs / rhs,
        Operator::Exponentiate => lhs.powf(rhs),
    }
}

impl Evaluate for ASTNode {
    fn eval(self) -> EvaluationResult {
        match self.value {
            Value::Operator(operator) => eval_node(operator, *self.lhs.unwrap(), *self.rhs.unwrap()),
            Value::Number(Number{ value }) => value,
            Value::Nothing => 0.0,
        }
    }
}

impl fmt::Debug for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Value::Operator(ref op) => write!(f, "{:?} {:?} {:?}", self.lhs, op, self.rhs),
            Value::Number(Number{ value }) => write!(f, "{}", value),
            Value::Nothing => write!(f, ""),
        }
    }
}

pub fn parse(expr: &str) -> ASTNode {
    let tokens = tokenize(&expr);

    //let output_queue: VecDeque
    let mut operator_stack: VecDeque<&Operator> = VecDeque::new();
    let mut operand_stack: VecDeque<ASTNode> = VecDeque::new();

    for token in &tokens {
        match token {
            Token::Number(Number { value }) => {
               operand_stack.push_back(ASTNode {
                   value: Value::Number(Number::new(*value)),
                   lhs: None,
                   rhs: None

                })
            }
            Token::Operator(op) => {
                operator_stack.push_back(&op);
            },
            _ => {}
        }
    }

    operand_stack.pop_front().unwrap()
}
