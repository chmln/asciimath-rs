use ast::{Node, Scope};
use std::collections::VecDeque;

use tokens::{Operator, Token};

use functions::FUNCTIONS;

pub type EvaluationResult = Result<f64, String>;
pub type FunctionArgs = VecDeque<Node>;

pub trait Evaluate {
    /// Evaluates the node/expression with a given variable scope.
    ///
    fn eval_with(self, scope: &Scope) -> EvaluationResult;

    /// Evaluates the node/expression without any variables.
    ///
    /// This is just a shortcut to evaluate expressions without variables.
    fn eval(self) -> EvaluationResult;
}

pub fn eval_operator(
    operator: &Operator,
    args: VecDeque<Node>,
    scope: &Scope,
) -> Result<f64, String> {
    println!("ARGS {:?}", args);
    let ref m_evaled_args: Result<Vec<_>, _> = args.into_iter()
        .map(|node| node.eval_with(scope))
        .collect();

    if let Ok(ok_args) = m_evaled_args {
        let mut evaled_args = ok_args.iter();
        match operator {
            Operator::Add => Ok(evaled_args.sum()),
            Operator::Substract => {
                Ok(evaled_args.nth(0).unwrap() - evaled_args.sum::<f64>())
            },
            Operator::Multiply => Ok(evaled_args.product()),
            Operator::Divide => {
                Ok(evaled_args.nth(0).unwrap() / evaled_args.product::<f64>())
            },
            Operator::Exponentiate => {
                let base = evaled_args.nth(0).unwrap();
                Ok(evaled_args
                    .by_ref()
                    .fold(*base, |acc, v| acc.powf(*v)))
            },
        }
    }
    else {
        Err(format!(
            "failed to evaluate {:?} {:?}",
            operator, m_evaled_args
        ))
    }
}

impl Evaluate for Node {
    fn eval_with(self, scope: &Scope) -> EvaluationResult {
        match self.token {
            Token::Operator(operator) => eval_operator(
                &operator,
                self.args.expect("operator must have args"),
                scope,
            ),
            Token::Function(f) => {
                if let (Some(f), Ok(args)) = (
                    FUNCTIONS.get(&f.name.as_ref()),
                    self.args
                        .unwrap()
                        .into_iter()
                        .map(|n| n.eval_with(scope))
                        .collect::<Result<Vec<f64>, _>>(),
                ) {
                    f(&args)
                }
                else {
                    Err(format!("Invalid function: {}", f.name))
                }
            },

            Token::Number(num) => Ok(num.value),
            Token::Variable(var) => {
                if let Some(value) = scope.get_var(&var.name) {
                    Ok(value.clone())
                }
                else {
                    Err(format!("Variable not found: {}", var.name))
                }
            },
            _ => Err(format!(
                "token should not be eval'd: {:?}",
                self.token
            )),
        }
    }

    fn eval(self) -> EvaluationResult {
        let empty_scope = Scope::new();
        self.eval_with(&empty_scope)
    }
}
