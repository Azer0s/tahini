#[cfg(test)]
mod tests {
    use crate::ast::{Literal, TopLevelStatement, VarInstruction, VarType};
    use crate::parser::parser;
    use chumsky::Parser;

    #[test]
    fn test_parse_top_level_var_def() {
        let input = 
        "(def a 10)\n
        (def b (ptr i32))\n
        (type my_type [i32, 10])";
        
        let result = parser().parse(input).into_output().unwrap();
        
        assert_eq!(result.len(), 3);
        
        assert_eq!(result[0], TopLevelStatement::DefVar("a".to_string(), VarInstruction::Literal(Literal::Int(10))));
        assert_eq!(result[1], TopLevelStatement::DefVar("b".to_string(), VarInstruction::Typed(VarType::Ptr(Box::new(VarType::Int32)))));
        assert_eq!(result[2], TopLevelStatement::TypeAlias("my_type".to_string(), VarType::ArraySized(Box::new(VarType::Int32), 10)));
    }
    
    #[test]
    fn test_parse_top_level_use() {
        let input = 
        "(def stdio (use :header \"stdio.h\"))\n
        (def my_module (use \"my_module\"))";
        
        let result = parser().parse(input).into_output().unwrap();
        
        assert_eq!(result.len(), 2);
        
        assert_eq!(result[0], TopLevelStatement::UseHeader("stdio".to_string(), "stdio.h".to_string()));
        assert_eq!(result[1], TopLevelStatement::Use("my_module".to_string(), "my_module".to_string()));
    }
}