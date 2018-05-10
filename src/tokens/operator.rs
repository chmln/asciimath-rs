use std::{cmp, fmt};

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

    pub fn num_operands(&self) -> i8 {
        match self {
            Operator::Not => 1,
            _ => 2,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        *self == Operator::Exponentiate
    }
}
