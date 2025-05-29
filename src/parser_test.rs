#[cfg(test)]
mod tests {
    use crate::ast::{DefVar, Literal, Statement, TopLevelDef, TopLevelStatement, VarType};
    use crate::parser::parser;
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
}
