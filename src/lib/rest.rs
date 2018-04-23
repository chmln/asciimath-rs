use lib::lexer::{Operator};
use std::{fmt};



pub enum Value {
    Node {
        operator: Operator,
        lhs: Box<Value>,
        rhs: Box<Value>,
    },

    Literal {
        value: f64,
    },

    Nothing,
}

type EvaluationResult = f64;

pub trait Evaluate {
    fn eval(self) -> EvaluationResult;
}

pub fn eval_node(operator: Operator, lhs_val: Value, rhs_val: Value) -> EvaluationResult {
    let lhs = lhs_val.eval();
    let rhs = rhs_val.eval();

    match operator {
        Operator::Add => lhs + rhs,
        Operator::Substract => lhs - rhs,
        Operator::Multiply => lhs * rhs,
        Operator::Divide => lhs / rhs,
    }
}

impl Evaluate for Value {
    fn eval(self) -> EvaluationResult {
        match self {
            Value::Node { operator, lhs, rhs } => eval_node(operator, *lhs, *rhs),
            Value::Literal { value } => value,
            Value::Nothing => 0.0,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Node { operator, lhs, rhs } => write!(f, "{:?} {:?} {:?}", lhs, operator, rhs),
            Value::Literal { value } => write!(f, "{}", value),
            Value::Nothing => write!(f, ""),
        }
    }
}

pub fn parse_expr(expr: &str) -> Box<Value> {
    let s = expr.clone().replace(" ", "");
    let mut i = 0;

    let check_num = expr.parse::<f64>();
    if check_num.is_ok() {
        return Box::new(Value::Literal {
            value: check_num.unwrap(),
        });
    }

    for c in s.chars() {
        match c {
            '+' => return add_or_subtract(Operator::Add, &s, i),
            '-' => return add_or_subtract(Operator::Substract, &s, i),
            '*' => {
                if !s.contains("+") && !s.contains("-") {
                    return multiply_or_divide(Operator::Multiply, &s, i);
                }
            }
            '/' => {
                if !s.contains("+") && !s.contains("-") {
                    return multiply_or_divide(Operator::Divide, &s, i);
                }
            }
            _ => (),
        }

        i += 1;
    }

    return Box::new(Value::Nothing);
}

pub fn add_or_subtract(operator: Operator, expr: &str, i: usize) -> Box<Value> {
    let lhs = parse_expr(&expr[0..i]);
    let rhs = parse_expr(&expr[(i + 1)..(expr.len())]);

    println!("lsh:({:?}) + rhs:({:?})", lhs, rhs);

    Box::new(Value::Node { operator, lhs, rhs })
}

pub fn multiply_or_divide(operator: Operator, expr: &str, i: usize) -> Box<Value> {
    let lhs = parse_expr(&expr[0..i]);
    let rhs = parse_expr(&expr[(i + 1)..(expr.len())]);

    println!("lsh:({:?}) * rhs:({:?})", lhs, rhs);
    Box::new(Value::Node { operator, lhs, rhs })
}


