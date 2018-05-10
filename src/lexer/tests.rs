#[allow(unused_imports)]
mod test {
    use ast::Scope;
    use lexer::tokenize;
    use tokens::{Function, Number, Operator, Token, Variable};
    #[test]
    fn lexer_negative_numbers() {
        let tokens = tokenize("x+-1", &Scope::new()).unwrap();
        let expected_tokens = vec![
            Token::Variable(Variable {
                name: "x".to_string(),
            }),
            Token::Operator(Operator::Add),
            Token::Number(Number::new(-1.0)),
            Token::Operator(Operator::Multiply),
            Token::Number(Number::new(1.0)),
        ];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_resolve_vars() {
        let vars = scope!{
            "abcd" => 1
        };

        let vars_2 = scope!{
            "ab" => 2,
            "cd" => 3
        };

        let vars_3 = scope!{
            "abc" => 2,
            "d" => 3
        };

        let vars_4 = scope!{
            "a" => 2,
            "bcd" => 3
        };

        assert_eq!(
            tokenize("abcd", &vars).unwrap(),
            vec![Token::Variable(Variable {
                name: "abcd".to_string(),
            })]
        );

        assert_eq!(
            tokenize("abcd", &vars_2).unwrap(),
            vec![
                Token::Variable(Variable {
                    name: "ab".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "cd".to_string(),
                }),
            ]
        );

        assert_eq!(
            tokenize("abcd", &vars_3).unwrap(),
            vec![
                Token::Variable(Variable {
                    name: "abc".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "d".to_string(),
                }),
            ]
        );

        assert_eq!(
            tokenize("abcd", &vars_4).unwrap(),
            vec![
                Token::Variable(Variable {
                    name: "a".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "bcd".to_string(),
                }),
            ]
        );

        assert_eq!(
            tokenize("abcd", &scope!{}).unwrap(),
            vec![
                Token::Variable(Variable {
                    name: "a".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "b".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "c".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "d".to_string(),
                }),
            ]
        );
    }

    #[test]
    fn lexer_word_variables() {
        assert_eq!(
            tokenize("quantity*2", &scope!{ "quantity" => 1 }),
            Ok(vec![
                Token::Variable(Variable {
                    name: "quantity".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::Number(Number::new(2.0)),
            ])
        );

        assert_eq!(
            tokenize("2quantity", &scope!{ "quantity" => 1 }),
            Ok(vec![
                Token::Number(Number::new(2.0)),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "quantity".to_string(),
                }),
            ])
        );
    }

    #[test]
    fn test_implicit_multiplication() {
        let mut scope = scope!{ "x" => 0 };

        assert_eq!(
            tokenize("1", &Scope::new()).unwrap(),
            vec![Token::Number(Number::new(1.0))]
        );
        assert_eq!(
            tokenize("3x^2", &Scope::new()).unwrap(),
            vec![
                Token::Number(Number::new(3)),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "x".to_string(),
                }),
                Token::Operator(Operator::Exponentiate),
                Token::Number(Number::new(2)),
            ]
        );
        assert_eq!(
            tokenize("4(x+3)2", &Scope::new()).unwrap(),
            vec![
                Token::Number(Number::new(4)),
                Token::Operator(Operator::Multiply),
                Token::LeftParenthesis,
                Token::Variable(Variable {
                    name: "x".to_string(),
                }),
                Token::Operator(Operator::Add),
                Token::Number(Number::new(3)),
                Token::RightParenthesis,
                Token::Operator(Operator::Multiply),
                Token::Number(Number::new(2)),
            ]
        );

        assert_eq!(
            tokenize("2x(x+3)", &scope).unwrap(),
            vec![
                Token::Number(Number::new(2)),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "x".to_string(),
                }),
                Token::Operator(Operator::Multiply),
                Token::LeftParenthesis,
                Token::Variable(Variable {
                    name: "x".to_string(),
                }),
                Token::Operator(Operator::Add),
                Token::Number(Number::new(3)),
                Token::RightParenthesis,
            ]
        );
        assert_eq!(
            tokenize("x^(2y+3z)", &Scope::new()).unwrap(),
            vec![
                Token::Variable(Variable {
                    name: "x".to_string(),
                }),
                Token::Operator(Operator::Exponentiate),
                Token::LeftParenthesis,
                Token::Number(Number::new(2)),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "y".to_string(),
                }),
                Token::Operator(Operator::Add),
                Token::Number(Number::new(3)),
                Token::Operator(Operator::Multiply),
                Token::Variable(Variable {
                    name: "z".to_string(),
                }),
                Token::RightParenthesis,
            ]
        )
    }

    #[test]
    fn lexer_floats() {
        let tokens = tokenize("max(1,3,25.75,10.5)", &Scope::new()).unwrap();
        let expected_tokens = vec![
            Token::Function(Function::new("max".to_string())),
            Token::Number(Number::new(1.0)),
            Token::Comma,
            Token::Number(Number::new(3.0)),
            Token::Comma,
            Token::Number(Number::new(25.75)),
            Token::Comma,
            Token::Number(Number::new(10.5)),
            Token::RightParenthesis,
        ];
        assert_eq!(tokens, expected_tokens)
    }
}
