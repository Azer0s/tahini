#[cfg(test)]
mod tests {
    use crate::ast::Literal;
    use crate::parser::literal;
    use chumsky::Parser;

    #[test]
    fn test_literal_int() {
        let input = "42";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::Int(42));
    }

    #[test]
    fn test_literal_float() {
        let input = "0.1312";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::Float(0.1312));
    }

    #[test]
    fn test_literal_bool() {
        let input = "true";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::Bool(true));
    }

    #[test]
    fn test_literal_char() {
        let input = "'c'";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::Char('c'));
    }

    #[test]
    fn test_literal_string() {
        let input = "\"hello\"";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::String("hello".to_string()));
    }

    #[test]
    fn test_literal_atom() {
        let input = ":my_atom";
        let result = literal().parse(input).into_output().unwrap();
        assert_eq!(result, Literal::Atom("my_atom".to_string()));
    }
}