use crate::{
    ast::{NumericLiteral, Value},
    Error, Result,
};
use std::{cmp, fmt};

pub type TokenList = Vec<Token>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(NumericLiteral),
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Function(String),
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
    Exponentiate,
    IsGreaterThan,
    IsLessThan,
    IsGreaterThanOrEqualTo,
    IsLessThanOrEqualTo,
    IsEqualTo,
    IsNotEqualTo,
    Not,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Substract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Exponentiate => "^",
                Operator::IsGreaterThan => ">",
                Operator::IsLessThan => "<",
                Operator::IsGreaterThanOrEqualTo => ">=",
                Operator::IsLessThanOrEqualTo => "<=",
                Operator::IsEqualTo => "==",
                Operator::IsNotEqualTo => "!=",
                Operator::Not => "!",
            }
        )
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl cmp::PartialEq for Operator {
    fn eq(&self, other: &Operator) -> bool {
        self.get_precedence() == other.get_precedence()
    }
}
impl cmp::PartialOrd for Operator {
    fn partial_cmp(&self, other: &Operator) -> Option<cmp::Ordering> {
        Some(self.get_precedence().cmp(&other.get_precedence()))
    }
}

impl Operator {
    pub fn get_precedence(&self) -> i8 {
        match self {
            Operator::Add => 2,
            Operator::Substract => 2,
            Operator::Multiply => 3,
            Operator::Divide => 3,
            Operator::Exponentiate => 4,
            Operator::IsGreaterThan
            | Operator::IsLessThan
            | Operator::IsGreaterThanOrEqualTo
            | Operator::IsLessThanOrEqualTo
            | Operator::IsEqualTo
            | Operator::IsNotEqualTo
            | Operator::Not => 2,
        }
    }

    pub fn num_operands(&self) -> usize {
        match self {
            Operator::Not => 1,
            _ => 2,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        *self == Operator::Exponentiate
    }

    pub fn eval(&self, args: &[Value]) -> Result<Value> {
        let args = &mut args.iter();

        match self {
            Operator::Not => {
                match args.next().ok_or(Error::MissingOperands(self.clone()))? {
                    Value::Num(x) => Ok(Value::Bool(x == &0.0)),
                    Value::Bool(a) => Ok(Value::Bool(!a)),
                }
            }
            _ => {
                let op1 =
                    args.next().ok_or(Error::MissingOperands(self.clone()))?;

                let op2 =
                    args.next().ok_or(Error::MissingOperands(self.clone()))?;

                match (self, op1, op2) {
                    (Operator::Add, Value::Num(n1), Value::Num(n2)) => {
                        Ok(Value::Num(n1 + n2))
                    }
                    (Operator::Substract, Value::Num(n1), Value::Num(n2)) => {
                        Ok(Value::Num(n1 - n2))
                    }
                    (Operator::Multiply, Value::Num(n1), Value::Num(n2)) => {
                        Ok(Value::Num(n1 * n2))
                    }
                    (Operator::Divide, Value::Num(n1), Value::Num(n2)) => {
                        Ok(Value::Num(n1 / n2))
                    }
                    (
                        Operator::Exponentiate,
                        Value::Num(n1),
                        Value::Num(n2),
                    ) => Ok(Value::Num(n1.powf(*n2))),
                    (Operator::IsGreaterThan, _, _) => {
                        Ok(Value::Bool(op1 > op2))
                    }
                    (Operator::IsLessThan, _, _) => Ok(Value::Bool(op1 < op2)),
                    (Operator::IsGreaterThanOrEqualTo, _, _) => {
                        Ok(Value::Bool(op1 >= op2))
                    }
                    (Operator::IsLessThanOrEqualTo, _, _) => {
                        Ok(Value::Bool(op1 <= op2))
                    }
                    (Operator::IsNotEqualTo, _, _) => {
                        Ok(Value::Bool(op1 != op2))
                    }
                    (Operator::IsEqualTo, lhs, rhs) => {
                        Ok(Value::Bool(lhs == rhs))
                    }
                    _ => Err(Error::InvalidOperands(self.clone())),
                }
            }
        }
    }
}
