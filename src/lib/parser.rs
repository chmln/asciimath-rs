use lexer::tokenize;
use std::{collections::HashMap, fmt};
use tokens::{Number, Operator, Token, Variable};

pub enum Value {
    Number(Number),
    Token(Token),
    Variable(Variable),
}

pub struct ASTNode {
    value: Value,
    lhs: Option<Box<ASTNode>>,
    rhs: Option<Box<ASTNode>>,
}

pub struct Scope {
    variables: HashMap<String, f64>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
        }
    }

    pub fn set_var<T>(&mut self, var_name: &str, value: T) -> ()
    where
        T: Copy + Into<f64> + PartialOrd + Clone,
    {
        self.variables
            .insert(var_name.to_string(), value.into());
    }

    pub fn get_var(&self, var_name: &str) -> Option<&f64> {
        self.variables.get(var_name)
    }
}

type EvaluationResult = Result<f64, String>;

pub trait Evaluate {
    fn eval_with(self, scope: &Scope) -> EvaluationResult;
    fn eval(self) -> EvaluationResult;
}

pub fn eval_node(
    operator: &Operator,
    lhs_val: ASTNode,
    rhs_val: ASTNode,
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

impl Evaluate for ASTNode {
    fn eval_with(self, scope: &Scope) -> EvaluationResult {
        match self.value {
            Value::Token(token) => match token {
                Token::Operator(operator) => eval_node(
                    &operator,
                    *self.lhs.unwrap(),
                    *self.rhs.unwrap(),
                    scope,
                ),
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

impl fmt::Debug for ASTNode {
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

pub fn parse(expr: &str) -> Option<ASTNode> {
    let tokens = tokenize(&expr);

    let mut operator_stack: Vec<Token> = Vec::new();
    let mut operand_stack: Vec<ASTNode> = Vec::new();

    let add_node = |lhs, rhs, token: Token| ASTNode {
        value: Value::Token(token),
        lhs: Some(Box::new(lhs)),
        rhs: Some(Box::new(rhs)),
    };

    for token in tokens {
        //println!("token: {:?}", token);
        match token {
            Token::Number(num) => {
                operand_stack.push(ASTNode {
                    value: Value::Number(Number {
                        value: num.value,
                    }),
                    lhs: None,
                    rhs: None,
                });
                //println!("Add number to output: {:?}", num);
            }
            Token::Variable(var) => operand_stack.push(ASTNode {
                value: Value::Variable(var),
                lhs: None,
                rhs: None,
            }),
            Token::RightParenthesis => while !operator_stack.is_empty() {
                let top = operator_stack.pop().unwrap();
                match top {
                    Token::LeftParenthesis => {
                        break;
                    }
                    Token::Operator(op) => {
                        let rhs = operand_stack.pop().expect("missing operand");
                        let lhs = operand_stack.pop().expect("missing operand");
                        operand_stack.push(add_node(
                            lhs,
                            rhs,
                            Token::Operator(op),
                        ));
                    }
                    _ => {}
                }
            },

            Token::LeftParenthesis => {
                operator_stack.push(token);
            }

            Token::Operator(op1) => {
                while !operator_stack.is_empty() {
                    let top = operator_stack.pop().unwrap();

                    match top {
                        Token::Operator(top_operator) => {
                            if top_operator > op1
                                || (top_operator == op1
                                    && !op1.is_right_associative())
                            {
                                let rhs = operand_stack.pop().unwrap();
                                let lhs = operand_stack.pop().unwrap();
                                //println!("{:?} {:?} {:?}", lhs, top_operator, rhs);
                                operand_stack.push(add_node(
                                    lhs,
                                    rhs,
                                    Token::Operator(top_operator),
                                ));
                            }
                            else {
                                operator_stack
                                    .push(Token::Operator(top_operator));
                                break;
                            }
                        }
                        _ => {
                            operator_stack.push(top);
                            break;
                            //println!("{:?}", top);
                        }
                    }
                }

                //println!("Push op to stack: {:?}", op1);
                operator_stack.push(Token::Operator(op1));
            }
        };
        // println!("stack: {:?}", operator_stack);
        //println!("output: {:?}", operand_stack);
        // println!("----------");
    }

    while !operator_stack.is_empty() {
        let rhs = operand_stack.pop().unwrap();
        let lhs = operand_stack.pop().unwrap();
        let operator = operator_stack.pop().unwrap();
        operand_stack.push(add_node(lhs, rhs, operator));
    }

    //println!("{:?}", operand_stack);

    operand_stack.pop()
}
