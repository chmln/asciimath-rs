use crate::{
    ast::{resolve_fn, resolve_var, Args, Node, Root, Scope},
    error::Error,
    tokens::{Operator, Token},
    util::Result,
};
use std::f64::EPSILON;

pub type NumericLiteral = f64;

pub type EvaluationResult = Result<NumericLiteral>;

pub trait Evaluate {
    /// Evaluates the node/expression with a given variable scope.
    fn eval_with(&self, scope: &Scope) -> EvaluationResult;

    /// Evaluates the node/expression without any variables.
    ///
    /// This is just a shortcut to evaluate expressions without variables.
    fn eval(&self) -> EvaluationResult;
}

fn negate(n: f64) -> f64 {
    if n == 0.0 {
        1.0
    }
    else {
        0.0
    }
}

fn int(b: bool) -> f64 {
    if b {
        1.0
    }
    else {
        0.0
    }
}

pub fn eval_operator(
    operator: &Operator,
    args: &[NumericLiteral],
) -> EvaluationResult {
    let evaled_args = &mut args.iter();
    let op_str = format!("{:?}", operator);

    match operator {
        Operator::Add => Ok(evaled_args.sum()),
        Operator::Substract => Ok(evaled_args
            .nth(0)
            .ok_or_else(|| Error::MissingOperands(op_str))?
            - evaled_args.sum::<NumericLiteral>()),
        Operator::Multiply => Ok(evaled_args.product()),
        Operator::Divide => Ok(evaled_args
            .nth(0)
            .ok_or_else(|| Error::MissingOperands(op_str))?
            / evaled_args.product::<NumericLiteral>()),
        Operator::Exponentiate => {
            let base = evaled_args
                .nth(0)
                .ok_or_else(|| Error::MissingOperands(op_str))?;
            Ok(evaled_args.fold(*base, |acc, v| acc.powf(*v)))
        },
        Operator::IsGreaterThan => Ok(int(args[0] > args[1])),
        Operator::IsLessThan => Ok(int(args[0] < args[1])),
        Operator::IsGreaterThanOrEqualTo => Ok(int(args[0] >= args[1])),
        Operator::IsLessThanOrEqualTo => {
            Ok(((args[0] <= args[1]) as i8).into())
        },
        Operator::IsEqualTo => Ok(int((args[0] - args[1]).abs() < EPSILON)),
        Operator::IsNotEqualTo => Ok(int((args[0] - args[1]).abs() > EPSILON)),
        Operator::Not => Ok(negate(args[0])),
    }
}

fn eval_args(
    args: &Option<Args>,
    scope: &Scope,
    fn_name: String,
) -> Result<Vec<NumericLiteral>> {
    if let Some(args) = args {
        return args.iter().map(|n| n.eval_with(scope)).collect();
    }
    Err(Error::NotEnoughFunctionParams(fn_name))
}

impl Evaluate for Node {
    fn eval_with(&self, scope: &Scope) -> EvaluationResult {
        match self.token {
            Token::Operator(ref operator) => {
                dbg!(&self.args);
                let args = self
                    .args
                    .as_ref()
                    .ok_or_else(|| {
                        Error::MissingOperands(format!("{:?}", operator))
                    })?
                    .iter()
                    .map(|node| node.eval_with(scope))
                    .collect::<Result<Vec<NumericLiteral>>>()?;

                dbg!(&args);
                let res = eval_operator(&operator, &args);
                dbg!(res)
            },
            Token::Function(ref f) => {
                let args = eval_args(&self.args, scope, f.clone())?;
                resolve_fn(f, scope)?(&args)
            },

            Token::Number(num) => Ok(num),
            Token::Variable(ref var) => resolve_var(&var, scope),
            _ => Err(Error::CannotEvaluateToken(format!("{:?}", self.token))),
        }
    }

    fn eval(&self) -> EvaluationResult {
        self.eval_with(&Scope::new())
    }
}

impl<'a> Evaluate for Root<'a> {
    fn eval(&self) -> EvaluationResult {
        self.node.eval_with(self.scope)
    }

    fn eval_with(&self, scope: &Scope) -> EvaluationResult {
        self.node.eval_with(scope)
    }
}
