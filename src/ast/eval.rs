use crate::{
    ast::{resolve_fn, resolve_var, Args, Node, Root, Scope},
    constants::Args as FnArgs,
    error::Error,
    tokens::Token,
    Result,
};

pub type NumericLiteral = f64;
pub type EvaluationResult = Result<Value>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Value {
    Num(f64),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(x) => {
                if let Some(prec) = f.precision() {
                    write!(f, "{1:.*}", prec, x)
                } else {
                    write!(f, "{}", x)
                }
            }
            Value::Bool(b) => f.write_str(&b.to_string()),
        }
    }
}

impl From<f64> for Value {
    fn from(num: f64) -> Self {
        Value::Num(num)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

pub trait Evaluate {
    /// Evaluates the node/expression with a given variable scope.
    fn eval_with(&self, scope: &Scope) -> Result<Value>;

    /// Evaluates the node/expression without any variables.
    ///
    /// This is just a shortcut to evaluate expressions without variables.
    fn eval(&self) -> Result<Value> {
        self.eval_with(&Scope::new())
    }
}

fn eval_args(
    args: &Option<Args>,
    scope: &Scope,
    fn_name: &str,
) -> Result<FnArgs> {
    match args {
        Some(args) => Ok(args
            .into_iter()
            .map(|n| n.eval_with(scope))
            .collect::<Result<Vec<_>>>()?
            .into()),
        None => Err(Error::NotEnoughFunctionParams(fn_name.to_owned())),
    }
}

impl Evaluate for Node {
    fn eval_with(&self, scope: &Scope) -> EvaluationResult {
        match self.token {
            Token::Operator(ref operator) => {
                let args = self
                    .args
                    .as_ref()
                    .ok_or(Error::MissingOperands(operator.clone()))?
                    .iter()
                    .map(|node| node.eval_with(scope))
                    .collect::<Result<Vec<Value>>>()?;

                operator.eval(&args)
            }
            Token::Function(ref fn_name) => {
                let args = eval_args(&self.args, scope, fn_name)?;
                let func = resolve_fn(fn_name, scope)?;
                func(args)
            }
            Token::Number(x) => Ok(Value::Num(x)),
            Token::Variable(ref var) => resolve_var(&var, scope),
            _ => Err(Error::CannotEvaluateToken(format!("{:?}", self.token))),
        }
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
