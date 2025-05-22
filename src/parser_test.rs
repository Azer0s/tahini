#[cfg(test)]
mod tests {
    use crate::ast::{DefVar, Literal, TopLevelDef, TopLevelStatement, VarType};
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
    fn test_parse_top_level_function_def() {
        let input = "(def main (fn [] i32 (do \n
            (def a 10)\n
        )))";

        let result = parser().parse(input).into_output().unwrap();

        println!("{:?}", result);

        let input = "(def main (fn [] i32 (do \n
            (printf \"Hello\") \n
            (printf \", world\n\")\
        )))";

        let result = parser().parse(input).into_output().unwrap();
        println!("{:?}", result);
    }
}
