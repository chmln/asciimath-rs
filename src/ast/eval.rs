use ast::{Args, Node, Scope, Variable};
use functions::{Func, FUNCTIONS};
use tokens::{Operator, Token};

pub type NumericLiteral = f64;
pub type EvaluationResult = Result<NumericLiteral, String>;

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
    args: Option<Args>,
    scope: &Scope,
) -> EvaluationResult {
    let args = args.ok_or("operator must have args")?
        .into_iter()
        .map(|node| node.eval_with(scope))
        .collect::<Result<Vec<NumericLiteral>, String>>()?;

    let ref mut evaled_args = args.iter();

    match operator {
        Operator::Add => Ok(evaled_args.sum()),
        Operator::Substract => Ok(evaled_args.nth(0).ok_or(format!(
            "Not enough arguments for operator {:?}",
            operator
        ))?
            - evaled_args.sum::<NumericLiteral>()),
        Operator::Multiply => Ok(evaled_args.product()),
        Operator::Divide => Ok(evaled_args.nth(0).ok_or(format!(
            "Not enough arguments for operator {:?}",
            operator
        ))?
            / evaled_args.product::<NumericLiteral>()),
        Operator::Exponentiate => {
            let base = evaled_args.nth(0).ok_or(format!(
                "Not enough arguments for operator {:?}",
                operator
            ))?;
            Ok(evaled_args.fold(*base, |acc, v| acc.powf(*v)))
        },
    }
}

fn get_fn<'a>(name: &str, scope: &'a Scope) -> Result<&'a Func, String> {
    FUNCTIONS.get(name).map_or_else(
        || match scope.get_var(name) {
            Some(Variable::Function(f)) => Ok(f),
            _ => Err(format!("Function \"{}\" is not defined", name)),
        },
        |f| Ok(f),
    )
}

fn eval_args(
    args: Option<Args>,
    scope: &Scope,
) -> Result<Vec<NumericLiteral>, String> {
    if let Some(args) = args {
        return args.into_iter()
            .map(|n| n.eval_with(scope))
            .collect::<Result<Vec<NumericLiteral>, _>>();
    }
    Err("Not enough arguments given".to_string())
}

impl Evaluate for Node {
    fn eval_with(self, scope: &Scope) -> EvaluationResult {
        match self.token {
            Token::Operator(operator) => {
                eval_operator(&operator, self.args, scope)
            },
            Token::Function(f) => {
                get_fn(&f.name.as_ref(), scope)?(&eval_args(self.args, scope)?)
            },

            Token::Number(num) => Ok(num.value),
            Token::Variable(var) => {
                if let Some(Variable::Number(value)) = scope.get_var(&var.name)
                {
                    return Ok(value.clone());
                }

                Err(format!("Variable not found: {}", var.name))
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
