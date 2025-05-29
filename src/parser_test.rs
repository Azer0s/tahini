#[cfg(test)]
mod tests {
    use crate::ast::{DefVar, FnDef, Literal, Statement, TopLevelDef, TopLevelStatement, VarType};
    use crate::parser::{parser, statement};
    use chumsky::Parser;

    #[test]
    fn test_parse_top_level_var_def() {
        let input = "(def a 10)\n
        (def b (ptr i32))\n
        (type my_type [i32 10])";

        let result = parser().parse(input).into_output().unwrap();
        assert_eq!(result.len(), 3);

        assert_eq!(
            result[0],
            TopLevelStatement::TopLevelDef(DefVar {
                name: "a".to_string(),
                instruction: TopLevelDef::Literal(Literal::Int(10))
            })
        );
        assert_eq!(
            result[1],
            TopLevelStatement::TopLevelDef(DefVar {
                name: "b".to_string(),
                instruction: TopLevelDef::Typed(VarType::Ptr(Box::new(VarType::Int32)))
            })
        );
        assert_eq!(
            result[2],
            TopLevelStatement::TypeAlias(
                "my_type".to_string(),
                VarType::ArraySized(Box::new(VarType::Int32), 10)
            )
        );
    }

    #[test]
    fn test_parse_top_level_use() {
        let input = "(def stdio (use :header \"stdio.h\"))\n
        (def my_module (use \"my_module\"))";

        let result = parser().parse(input).into_output().unwrap();

        assert_eq!(result.len(), 2);

        assert_eq!(
            result[0],
            TopLevelStatement::UseHeader("stdio".to_string(), "stdio.h".to_string())
        );
        assert_eq!(
            result[1],
            TopLevelStatement::Use("my_module".to_string(), "my_module".to_string())
        );
    }

    #[test]
    fn test_parse_top_level_function_def_1() {
        let input = "(def main (fn [] i32 (do \n
            (def a 10)\n
        )))";

        let result = parser().parse(input).into_output().unwrap();
        assert_eq!(result.len(), 1);

        if let TopLevelStatement::TopLevelDef(df) = &result[0] {
            if let TopLevelDef::FnDef(f) = &df.instruction {
                assert_eq!(f.return_type, VarType::Int32);
                assert_eq!(f.parameters.len(), 0);
                assert_eq!(
                    f.statement,
                    Statement::DoBlock(vec![Statement::DefVar(DefVar {
                        name: "a".to_string(),
                        instruction: Box::new(Statement::Literal(Literal::Int(10))),
                    })])
                );
            }
        }
    }

    #[test]
    fn test_parse_top_level_function_def_2() {
        let input = "(def main (fn [] i32 (do \n
            (printf \"Hello\") \n
            (printf \", world\n\")\
        )))";

        let result = parser().parse(input).into_output().unwrap();
        assert_eq!(result.len(), 1);

        if let TopLevelStatement::TopLevelDef(df) = &result[0] {
            if let TopLevelDef::FnDef(f) = &df.instruction {
                assert_eq!(f.return_type, VarType::Int32);
                assert_eq!(f.parameters.len(), 0);
                assert_eq!(
                    f.statement,
                    Statement::DoBlock(vec![
                        Statement::Call(
                            "printf".to_string(),
                            vec![Statement::Literal(Literal::String("Hello".to_string()))],
                        ),
                        Statement::Call(
                            "printf".to_string(),
                            vec![Statement::Literal(Literal::String(", world\n".to_string()))],
                        ),
                    ])
                );
            }
        }
    }

    #[test]
    fn test_parse_top_level_function_def_3() {
        let input = "(def fib (fn [(:n i32)] i32 \n
            (if (< n 2) \n
                n \n\
                (+ (fib (- n 1)) (fib (- n 2)))) \n
            ))";
        let result = parser().parse(input).into_output().unwrap();
        assert_eq!(result.len(), 1);
        if let TopLevelStatement::TopLevelDef(df) = &result[0] {
            if let TopLevelDef::FnDef(f) = &df.instruction {
                assert_eq!(f.return_type, VarType::Int32);
                assert_eq!(f.parameters.len(), 1);
                assert_eq!(f.parameters[0], ("n".to_string(), VarType::Int32));
                assert_eq!(
                    f.statement,
                    Statement::IfElse(
                        Box::new(Statement::Call(
                            "<".to_string(),
                            vec![
                                Statement::Ident("n".to_string()),
                                Statement::Literal(Literal::Int(2))
                            ],
                        )),
                        Box::new(Statement::Ident("n".to_string())),
                        Box::new(Statement::Call(
                            "+".to_string(),
                            vec![
                                Statement::Call(
                                    "fib".to_string(),
                                    vec![Statement::Call(
                                        "-".to_string(),
                                        vec![
                                            Statement::Ident("n".to_string()),
                                            Statement::Literal(Literal::Int(1))
                                        ],
                                    )],
                                ),
                                Statement::Call(
                                    "fib".to_string(),
                                    vec![Statement::Call(
                                        "-".to_string(),
                                        vec![
                                            Statement::Ident("n".to_string()),
                                            Statement::Literal(Literal::Int(2))
                                        ],
                                    )],
                                ),
                            ],
                        )),
                    )
                );
            }
        }
    }

    #[test]
    fn test_parse_top_level_function_def_4() {
        let input = "(def read-all (fn [] str \n
          (do \n
            (def result \"\") \n
            (def c (stdio/getchar)) \n
            (for (!= c (- 1)) \n
              (do \n
                (def result (str-append result (char-to-str c))) \n
                (def c (stdio/getchar)) \n
              ) \n
            ) \n
            result \n
          ) \n
        ))";
        let result = parser().parse(input).into_output().unwrap();
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_top_level_function_def_5() {
        let input = "(def main (fn [] i32 \n
            (do \n
                (def a 10) \n
                (def b 20) \n
                (if (< a b) \n
                    (printf \"a is less than b\") \n
                    (printf \"a is not less than b\") \n
                ) \n
            ) \n
        )) \n
        (export :all)";

        let result = parser().parse(input).into_output().unwrap();
        assert_eq!(result.len(), 2);

        if let TopLevelStatement::TopLevelDef(df) = &result[0] {
            if let TopLevelDef::FnDef(f) = &df.instruction {
                assert_eq!(f.return_type, VarType::Int32);
                assert_eq!(f.parameters.len(), 0);
                assert_eq!(
                    f.statement,
                    Statement::DoBlock(vec![
                        Statement::DefVar(DefVar {
                            name: "a".to_string(),
                            instruction: Box::new(Statement::Literal(Literal::Int(10))),
                        }),
                        Statement::DefVar(DefVar {
                            name: "b".to_string(),
                            instruction: Box::new(Statement::Literal(Literal::Int(20))),
                        }),
                        Statement::IfElse(
                            Box::new(Statement::Call(
                                "<".to_string(),
                                vec![
                                    Statement::Ident("a".to_string()),
                                    Statement::Ident("b".to_string())
                                ],
                            )),
                            Box::new(Statement::Call(
                                "printf".to_string(),
                                vec![Statement::Literal(Literal::String(
                                    "a is less than b".to_string()
                                ))],
                            )),
                            Box::new(Statement::Call(
                                "printf".to_string(),
                                vec![Statement::Literal(Literal::String(
                                    "a is not less than b".to_string()
                                ))],
                            )),
                        ),
                    ])
                );
            }
        }

        assert_eq!(result[1], TopLevelStatement::ExportAll());
    }

    #[test]
    fn test_parse_tuple_literal() {
        let input = "{1 2 3}";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::Literal(Literal::Tuple(vec![
                Statement::Literal(Literal::Int(1)),
                Statement::Literal(Literal::Int(2)),
                Statement::Literal(Literal::Int(3))
            ]))
        );
    }

    #[test]
    fn test_parse_data_literal() {
        let input = "[:some 10]";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::Literal(Literal::Data(
                "some".to_string(),
                vec![Statement::Literal(Literal::Int(10))]
            ))
        );
    }

    #[test]
    fn test_parse_array_literal() {
        let input = "[1 2 3]";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::Literal(Literal::Array(vec![
                Statement::Literal(Literal::Int(1)),
                Statement::Literal(Literal::Int(2)),
                Statement::Literal(Literal::Int(3))
            ]))
        );
    }

    #[test]
    fn test_parse_call() {
        let input = "(< a b)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::Call(
                "<".to_string(),
                vec![
                    Statement::Ident("a".to_string()),
                    Statement::Ident("b".to_string())
                ]
            )
        );
    }

    #[test]
    fn test_parse_generic_call() {
        let input = "(option<i8> :some 10)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::GenericCall(
                "option".to_string(),
                vec!["i8".to_string()],
                vec![
                    Statement::Literal(Literal::Atom("some".to_string())),
                    Statement::Literal(Literal::Int(10))
                ]
            )
        );
    }

    #[test]
    fn test_parse_dollar_operator() {
        let input = "($ [0] a)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::GetIndexed(
                Box::new(Statement::Literal(Literal::Int(0))),
                Box::new(Statement::Ident("a".to_string()))
            )
        );

        let input = "($ [0] a b)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::SetIndexed(
                Box::new(Statement::Literal(Literal::Int(0))),
                Box::new(Statement::Ident("a".to_string())),
                Box::new(Statement::Ident("b".to_string()))
            )
        );

        let input = "($ :x a)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::GetField("x".to_string(), Box::new(Statement::Ident("a".to_string())))
        );

        let input = "($ :x a 4.2)";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::SetField(
                "x".to_string(),
                Box::new(Statement::Ident("a".to_string())),
                Box::new(Statement::Literal(Literal::Float(4.2)))
            )
        );
    }

    #[test]
    fn test_parse_nested_dollar_operator() {
        let input = "($ :x ($ :y a))";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::GetField(
                "x".to_string(),
                Box::new(Statement::GetField(
                    "y".to_string(),
                    Box::new(Statement::Ident("a".to_string()))
                )),
            )
        );
    }

    #[test]
    fn test_fn_literal() {
        let input = "(fn [(:a i32) (:b f64)] i32 (if (< a b) a b))";
        let result = statement().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            Statement::Literal(Literal::Fn(Box::new(FnDef {
                generic_types: None,
                parameters: vec![
                    ("a".to_string(), VarType::Int32),
                    ("b".to_string(), VarType::Float64)
                ],
                return_type: VarType::Int32,
                statement: Statement::IfElse(
                    Box::new(Statement::Call(
                        "<".to_string(),
                        vec![
                            Statement::Ident("a".to_string()),
                            Statement::Ident("b".to_string())
                        ]
                    )),
                    Box::new(Statement::Ident("a".to_string())),
                    Box::new(Statement::Ident("b".to_string()))
                )
            })))
        );
    }
}
