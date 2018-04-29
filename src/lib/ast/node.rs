use ast::{Scope, Value};
use std::fmt;
use tokens::{Operator, Token};

pub struct Node {
    pub value: Value,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

pub type EvaluationResult = Result<f64, String>;

pub trait Evaluate {
    fn eval_with(self, scope: &Scope) -> EvaluationResult;
    fn eval(self) -> EvaluationResult;
}

pub fn eval_node(
    operator: &Operator,
    lhs_val: Node,
    rhs_val: Node,
    scope: &Scope,
) -> EvaluationResult {
    let ref lhs_result = lhs_val.eval_with(scope);
    let ref rhs_result = rhs_val.eval_with(scope);

    if let (Ok(lhs), Ok(rhs)) = (lhs_result, rhs_result) {
        match operator {
            Operator::Add => Ok(lhs + rhs),
            Operator::Substract => Ok(lhs - rhs),
            Operator::Multiply => Ok(lhs * rhs),
            Operator::Divide => Ok(lhs / rhs),
            Operator::Exponentiate => Ok(lhs.powf(*rhs)),
        }
    }
    else {
        Err(format!(
            "failed to evaluate {:?} {:?} {:?}",
            lhs_result, operator, rhs_result
        ))
    }
}

impl Evaluate for Node {
    fn eval_with(self, scope: &Scope) -> EvaluationResult {
        //println!("{:?}", self);
        match self.value {
            Value::Token(token) => match token {
                Token::Operator(operator) => eval_node(
                    &operator,
                    *self.lhs.unwrap(),
                    *self.rhs.unwrap(),
                    scope,
                ),
                Token::Function(f) => {
                    if f.name == "multiply_by_2" {
                        return Ok(self.rhs.unwrap().eval_with(scope).unwrap() * 2.0);
                    }
                    unimplemented!()
                }
                _ => Err(format!(
                    "token should not be eval'd: {:?}",
                    token
                )),
            },
            Value::Number(num) => Ok(num.value),
            Value::Variable(var) => {
                if let Some(value) = scope.get_var(&var.name) {
                    Ok(value.clone())
                }
                else {
                    Err(format!("Variable not found: {}", var.name))
                }
            }
        }
    }

    fn eval(self) -> EvaluationResult {
        let empty_scope = Scope::new();
        self.eval_with(&empty_scope)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Value::Token(ref token) => {
                write!(f, "{:?} {:?} {:?}", self.lhs, self.rhs, token)
            }
            Value::Variable(ref var) => write!(f, "{}", var.name),
            Value::Number(ref num) => write!(f, "{}", num.value),
        }
    }
}
