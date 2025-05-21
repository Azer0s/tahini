#[cfg(test)]
mod tests {
    use crate::ast::VarType;
    use crate::parser::var_type;
    use chumsky::Parser;

    #[test]
    fn test_type_int() {
        let input = "i32";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::Int32);
    }

    #[test]
    fn test_type_uint() {
        let input = "u16";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::UInt16);
    }

    #[test]
    fn test_type_float() {
        let input = "f64";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::Float64);
    }

    #[test]
    fn test_type_bool() {
        let input = "bool";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::Bool);
    }

    #[test]
    fn test_type_void() {
        let input = "void";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::Void);
    }

    #[test]
    fn test_type_array() {
        let input = "[i32 10]";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::ArraySized(Box::new(VarType::Int32), 10));
    }

    #[test]
    fn test_type_ptr() {
        let input = "(ptr i32)";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(result, VarType::Ptr(Box::new(VarType::Int32)));
    }

    #[test]
    fn test_type_tuple() {
        let input = "{i32 f64}";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            VarType::Tuple(vec![VarType::Int32, VarType::Float64])
        );
    }

    #[test]
    fn test_type_fn() {
        let input = "fn [ i32 f64 ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::Fn(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn [i32 f64] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::Fn(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn[i32 f64]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::Fn(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn[ i32 f64 ]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::Fn(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );
    }

    #[test]
    fn test_type_fn_varargs() {
        let input = "fn [ i32 f64 ... ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::FnWithVarArgs(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn [i32 f64 ...] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::FnWithVarArgs(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn[i32 f64 ...]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::FnWithVarArgs(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn[ i32 f64 ...]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::FnWithVarArgs(
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );
    }

    #[test]
    fn test_type_generic_fn() {
        let input = "fn<T> [ i32 f64 ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFn(
                vec!["T".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn<K V> [ i32 f64 ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFn(
                vec!["K".to_string(), "V".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn<K V>[i32 f64]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFn(
                vec!["K".to_string(), "V".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );
    }

    #[test]
    fn test_type_generic_fn_varargs() {
        let input = "fn<T> [ i32 f64 ... ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFnWithVarArgs(
                vec!["T".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn<K V> [ i32 f64 ... ] i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFnWithVarArgs(
                vec!["K".to_string(), "V".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );

        let input = "fn<K V>[i32 f64 ...]i32";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericFnWithVarArgs(
                vec!["K".to_string(), "V".to_string()],
                vec![VarType::Int32, VarType::Float64],
                Box::new(VarType::Int32)
            )
        );
    }

    #[test]
    fn test_type_struct() {
        let input = "(struct (:a i32) (:b f64))";
        let result = var_type().parse(input).into_output().unwrap();
        assert_eq!(
            result,
            VarType::Struct(vec![
                ("a".to_string(), VarType::Int32),
                ("b".to_string(), VarType::Float64),
            ])
        );
    }

    #[test]
    fn test_type_generic_struct() {
        let input = "(struct<T> (:a T) (:b f64))";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericStruct(
                vec!["T".to_string()],
                vec![
                    ("a".to_string(), VarType::IdentType("T".to_string())),
                    ("b".to_string(), VarType::Float64),
                ]
            )
        );

        let input = "(struct<K V> (:a K) (:b V))";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericStruct(
                vec!["K".to_string(), "V".to_string()],
                vec![
                    ("a".to_string(), VarType::IdentType("K".to_string())),
                    ("b".to_string(), VarType::IdentType("V".to_string())),
                ]
            )
        );
    }

    #[test]
    fn test_type_generic_array() {
        let input = "(array<T>)";
        let result = var_type().parse(input).unwrap();
        assert_eq!(result, VarType::GenericArrayUnsized("T".to_string()));
    }

    #[test]
    fn test_type_generic_array_sized() {
        let input = "(array<T> 10)";
        let result = var_type().parse(input).unwrap();
        assert_eq!(result, VarType::GenericArraySized("T".to_string(), 10));
    }

    #[test]
    fn test_type_generic_ptr() {
        let input = "(ptr<T>)";
        let result = var_type().parse(input).unwrap();
        assert_eq!(result, VarType::GenericPtr("T".to_string()));
    }

    #[test]
    fn test_type_data() {
        let input = "(data [:a i32] [:b f64])";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::Data(vec![
                ("a".to_string(), vec![VarType::Int32]),
                ("b".to_string(), vec![VarType::Float64]),
            ])
        );
    }

    #[test]
    fn test_type_generic_data() {
        let input = "(data<T> [:a i32] [:b f64])";
        let result = var_type().parse(input).unwrap();
        assert_eq!(
            result,
            VarType::GenericData(
                vec!["T".to_string()],
                vec![
                    ("a".to_string(), vec![VarType::Int32]),
                    ("b".to_string(), vec![VarType::Float64])
                ]
            )
        )
    }
}
