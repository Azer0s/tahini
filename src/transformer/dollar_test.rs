#[cfg(test)]
mod tests {
    use crate::ast::Statement;
    use crate::parser::statement;
    use crate::transformer::ast::AccessSegment;
    use crate::transformer::dollar::{try_linearize, DollarChain};
    use chumsky::Parser;

    fn parse(src: &str) -> Statement {
        statement().parse(src).unwrap()
    }

    #[test]
    fn test_simple_field_access() {
        let s = parse("($ :x a)");
        let chain = try_linearize(&s).expect("should detect");
        match chain {
            DollarChain::Access { root, segments } => {
                assert_eq!(*root, Statement::Ident("a".into()));
                assert_eq!(segments, vec![AccessSegment::Field("x".into())]);
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn test_nested_field_access() {
        let s = parse("($ :x ($ :y a))");
        let chain = try_linearize(&s).expect("detected");
        match chain {
            DollarChain::Access { root, segments } => {
                assert_eq!(*root, Statement::Ident("a".into()));
                assert_eq!(
                    segments,
                    vec![
                        AccessSegment::Field("x".into()),
                        AccessSegment::Field("y".into()),
                    ]
                );
            }
            _ => panic!("variant"),
        }
    }

    #[test]
    fn test_assignment() {
        let s = parse("($ :x ($ :y a) 10)");
        let chain = try_linearize(&s).expect("detected");
        match chain {
            DollarChain::Assign {
                root,
                segments,
                value,
            } => {
                assert_eq!(*root, Statement::Ident("a".into()));
                assert_eq!(
                    segments,
                    vec![
                        AccessSegment::Field("x".into()),
                        AccessSegment::Field("y".into()),
                    ]
                );
                assert_eq!(*value, Statement::Literal(crate::ast::Literal::Int(10)));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_mixed_segments() {
        let s = parse("($ [0] ($ :y a))");
        let chain = try_linearize(&s).expect("detected");
        match chain {
            DollarChain::Access { root, segments } => {
                assert_eq!(*root, Statement::Ident("a".into()));
                assert_eq!(
                    segments,
                    vec![
                        AccessSegment::Index(Box::new(Statement::Literal(
                            crate::ast::Literal::Int(0)
                        ))),
                        AccessSegment::Field("y".into()),
                    ]
                );
            }
            _ => panic!(),
        }
    }
}
