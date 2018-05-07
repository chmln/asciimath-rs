use ast::{resolve_fn, resolve_var, Args, Node, Root, Scope};
use error::Error;

use tokens::{Operator, Token};

pub type NumericLiteral = f64;

pub type EvaluationResult = Result<NumericLiteral, Error>;

pub trait Evaluate {
    /// Evaluates the node/expression with a given variable scope.
    ///
    fn eval_with(&self, scope: &Scope) -> EvaluationResult;

    /// Evaluates the node/expression without any variables.
    ///
    /// This is just a shortcut to evaluate expressions without variables.
    fn eval(&self) -> EvaluationResult;
}

pub fn eval_operator(
    operator: &Operator,
    args: &Option<Args>,
    scope: &Scope,
) -> EvaluationResult {
    let args = args.as_ref()
        .ok_or_else(|| Error::MissingOperands(format!("{:?}", operator)))?
        .into_iter()
        .map(|node| node.eval_with(scope))
        .collect::<Result<Vec<NumericLiteral>, Error>>()?;

    let ref mut evaled_args = args.iter();

    match operator {
        Operator::Add => Ok(evaled_args.sum()),
        Operator::Substract => Ok(evaled_args.nth(0).ok_or(
            Error::MissingOperands(format!("{:?}", operator)),
        )?
            - evaled_args.sum::<NumericLiteral>()),
        Operator::Multiply => Ok(evaled_args.product()),
        Operator::Divide => Ok(evaled_args.nth(0).ok_or(
            Error::MissingOperands(format!("{:?}", operator)),
        )?
            / evaled_args.product::<NumericLiteral>()),
        Operator::Exponentiate => {
            let base = evaled_args.nth(0).ok_or_else(|| {
                Error::MissingOperands(format!("{:?}", operator))
            })?;
            Ok(evaled_args.fold(*base, |acc, v| acc.powf(*v)))
        },
    }
}

fn eval_args(
    args: &Option<Args>,
    scope: &Scope,
    fn_name: String,
) -> Result<Vec<NumericLiteral>, Error> {
    if let Some(args) = args {
        return args.into_iter()
            .map(|n| n.eval_with(scope))
            .collect::<Result<Vec<NumericLiteral>, _>>();
    }
    Err(Error::NotEnoughFunctionParams(fn_name))
}

impl Evaluate for Node {
    fn eval_with(&self, scope: &Scope) -> EvaluationResult {
        match self.token {
            Token::Operator(ref operator) => {
                eval_operator(&operator, &self.args, scope)
            },
            Token::Function(ref f) => resolve_fn(&f.name.as_ref(), scope)?(
                &eval_args(&self.args, scope, f.name.clone())?,
            ),

            Token::Number(ref num) => Ok(num.value),
            Token::Variable(ref var) => {
                if let Ok(value) = resolve_var(&var.name, scope) {
                    return Ok(value.clone());
                }

                Err(Error::UnknownVariable(var.name.clone()))
            },
            _ => Err(Error::CannotEvaluateToken(format!(
                "{:?}",
                self.token
            ))),
        }
    }

    fn eval(&self) -> EvaluationResult {
        let empty_scope = Scope::new();
        self.eval_with(&empty_scope)
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
