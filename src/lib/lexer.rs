use std::{fmt};

pub enum Operator {
    Add,
    Substract,
    Multiply,
    Divide,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => "+",
                Operator::Substract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
            }
        )
    }
}

pub struct NumericLiteral {
    value: f64,
}

impl fmt::Debug for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
pub enum Token {
    Operator(Operator),
    NumericLiteral(NumericLiteral),
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let trimmed = expr.replace(" ", "");
    let mut chars = trimmed.chars();

    let mut temp = String::new();

    loop {
        if let Some(c) = chars.next() {
            if c.is_alphanumeric() {
                temp.push(c);
                continue;
            }

            if temp.len() > 0 {
                if let Ok(num) = temp.parse::<f64>() {
                    tokens.push(Token::NumericLiteral(NumericLiteral { value: num }))
                }

                temp.clear()
            }

            match c {
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Substract)),
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),

                // todo parens
                '(' => (),
                ')' => (),
                _ => {}
            }
        } else {
            // todo: refactor
            if temp.len() > 0 {
                if let Ok(num) = temp.parse::<f64>() {
                    tokens.push(Token::NumericLiteral(NumericLiteral { value: num }))
                }
            }

            break;
        }
    }

    tokens
}

pub fn postfix(expr: &str) -> String {
    let mut s = String::new();
    let tokens = tokenize(&expr);

    //let output_queue: VecDeque
    let mut operator_stack: Vec<&Operator> = Vec::new();

    for token in &tokens {
        match token {
            Token::NumericLiteral(NumericLiteral { value }) => {
                s.push_str(&*value.to_string());
            }
            Token::Operator(op) => {
                operator_stack.push(op);
            }
        }
    }

    s
}
