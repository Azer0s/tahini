use crate::ast::{DefVar, FnDef, Literal, Statement, TopLevelDef, TopLevelStatement, VarType};
use chumsky::prelude::*;

fn ident<'a>() -> impl Parser<'a, &'a str, String> + Clone {
    let keywords = [
        "def", "fn", "if", "do", "use", "array", "ptr", "data", "struct", "tuple", "...", "true",
        "false",
    ];

    any()
        .filter(|c: &char| {
            !c.is_whitespace()
                && *c != '('
                && *c != ')'
                && !c.is_numeric()
                && *c != '['
                && *c != ']'
                && *c != '{'
                && *c != '}'
                && *c != ':'
                && *c != ','
                && *c != '"'
                && *c != '\''
        })
        .filter(|c: &char| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .filter(move |s: &String| !keywords.contains(&s.as_str()))
}

pub fn literal<'a>() -> impl Parser<'a, &'a str, Literal> + Clone {
    let int = text::int(10).map(|s: &str| Literal::Int(s.parse().unwrap()));

    let float = text::int(10)
        .then_ignore(just("."))
        .then(text::int(10))
        .map(|(int_part, frac_part): (&'a str, &'a str)| {
            let float_str = format!("{}.{}", int_part, frac_part);
            Literal::Float(float_str.parse().unwrap())
        });

    let atom = just(":").ignore_then(ident()).map(Literal::Atom);

    let bool = just("true")
        .or(just("false"))
        .map(|s: &str| Literal::Bool(s == "true"));

    let char = just("'")
        .ignore_then(none_of("'"))
        .then_ignore(just("'"))
        .map(|c: char| Literal::Char(c));

    let string = just("\"")
        .ignore_then(none_of("\"").repeated().collect::<String>())
        .then_ignore(just("\""))
        .map(|s: String| Literal::String(s));

    float.or(int).or(bool).or(atom).or(char).or(string)
}

fn generics<'a>() -> impl Parser<'a, &'a str, Vec<String>> + Clone {
    just("<")
        .ignore_then(ident().padded().repeated().at_least(1).collect::<Vec<_>>())
        .then_ignore(just(">"))
        .padded()
}

pub fn var_type<'a>() -> impl Parser<'a, &'a str, VarType> + Clone {
    recursive(|var_type_rec| {
        // Basic types
        let basic_type = choice((
            just("i8").to(VarType::Int8),
            just("i16").to(VarType::Int16),
            just("i32").to(VarType::Int32),
            just("i64").to(VarType::Int64),
            just("i128").to(VarType::Int128),
            just("u8").to(VarType::UInt8),
            just("u16").to(VarType::UInt16),
            just("u32").to(VarType::UInt32),
            just("u64").to(VarType::UInt64),
            just("u128").to(VarType::UInt128),
            just("f16").to(VarType::Float16),
            just("f32").to(VarType::Float32),
            just("f64").to(VarType::Float64),
            just("f128").to(VarType::Float128),
            just("bool").to(VarType::Bool),
            just("void").to(VarType::Void),
        ));

        // Array
        let array_sized = just("[")
            .padded()
            .ignore_then(var_type_rec.clone().padded())
            .then(text::int(10).padded())
            .then_ignore(just("]"))
            .map(|(var_type, size): (VarType, &str)| {
                let size = size.parse::<usize>().unwrap();
                VarType::ArraySized(Box::from(var_type), size)
            });

        let array_unsized = just("[")
            .padded()
            .ignore_then(var_type_rec.clone().padded())
            .then_ignore(just("]"))
            .map(|var_type| VarType::ArrayUnsized(Box::from(var_type)));

        let array_start = just("(").padded().ignore_then(just("array"));
        let array_long_form = array_start
            .ignore_then(var_type_rec.clone().padded())
            .then(text::int(10).padded().or_not())
            .then_ignore(just(")"))
            .map(|(var_type, size): (VarType, Option<&'a str>)| match size {
                Some(size) => VarType::ArraySized(Box::from(var_type), size.parse().unwrap()),
                None => VarType::ArrayUnsized(Box::from(var_type)),
            });

        let generic_array_long_form = array_start
            .ignore_then(generics())
            .then(text::int(10).padded().or_not())
            .then_ignore(just(")"))
            .map(|(generic_types, size): (Vec<String>, Option<&'a str>)| {
                if generic_types.len() != 1 {
                    panic!("Generic array must have exactly one type parameter");
                }

                match size {
                    Some(size) => VarType::GenericArraySized(
                        generic_types[0].to_string(),
                        size.parse().unwrap(),
                    ),
                    None => VarType::GenericArrayUnsized(generic_types[0].to_string()),
                }
            });

        // Pointer
        let ptr = just("(")
            .padded()
            .ignore_then(just("ptr").padded())
            .ignore_then(var_type_rec.clone().padded())
            .then_ignore(just(")"))
            .map(|var_type| VarType::Ptr(Box::from(var_type)));

        // Generic pointer
        let generic_ptr = just("(")
            .padded()
            .ignore_then(just("ptr").padded())
            .ignore_then(generics())
            .then_ignore(just(")"))
            .map(|generic_types| {
                if generic_types.len() != 1 {
                    panic!("Pointer generics must have exactly one type parameter");
                }

                VarType::GenericPtr(generic_types[0].to_string())
            });

        // Tuple
        let tuple_shorthand = just("{")
            .padded()
            .ignore_then(
                var_type_rec
                    .clone()
                    .padded()
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just("}"))
            .map(VarType::Tuple);

        let tuple_longform = just("(")
            .padded()
            .ignore_then(just("tuple").padded())
            .ignore_then(generics().or_not())
            .then(
                var_type_rec
                    .clone()
                    .padded()
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(")"))
            .map(|(generic_types, types)| {
                if let Some(generics) = generic_types {
                    VarType::GenericTuple(generics.iter().map(|s| s.to_string()).collect(), types)
                } else {
                    VarType::Tuple(types)
                }
            });

        let tuple = tuple_shorthand.or(tuple_longform);

        // Function
        let fn_type = just("fn")
            .padded()
            .ignore_then(generics().or_not())
            .then(
                just("[")
                    .padded()
                    .ignore_then(var_type_rec.clone().padded().repeated().collect::<Vec<_>>())
                    .then(just("...").padded().or_not().map(|va| va.is_some()))
                    .then_ignore(just("]").padded()),
            )
            .then(var_type_rec.clone())
            .map(
                |((generic_types, (args, is_va)), ret)| match (generic_types, is_va) {
                    (Some(generics), true) => VarType::GenericFnWithVarArgs(
                        generics.iter().map(|s| s.to_string()).collect(),
                        args,
                        Box::new(ret),
                    ),
                    (None, true) => VarType::FnWithVarArgs(args, Box::new(ret)),
                    (Some(generics), false) => VarType::GenericFn(
                        generics.iter().map(|s| s.to_string()).collect(),
                        args,
                        Box::new(ret),
                    ),
                    (None, false) => VarType::Fn(args, Box::new(ret)),
                },
            );

        // Data
        let data_field = just("[")
            .padded()
            .ignore_then(just(":"))
            .ignore_then(ident())
            .then(var_type_rec.clone().padded().repeated().collect::<Vec<_>>())
            .then_ignore(just("]"));

        type DataParseType<'a> = (Option<Vec<String>>, Vec<(String, Vec<VarType>)>);

        let data = just("(")
            .padded()
            .ignore_then(just("data").padded())
            .ignore_then(generics().or_not())
            .then(
                data_field
                    .clone()
                    .padded()
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(")"))
            .map(|(generic_types, fields): DataParseType<'a>| {
                let mut field_names = std::collections::HashSet::new();
                for (name, _) in &fields {
                    if !field_names.insert(name.to_string()) {
                        panic!("Duplicate field name: {}", name);
                    }
                }

                if let Some(generics) = generic_types {
                    VarType::GenericData(
                        generics.iter().map(|s| s.to_string()).collect(),
                        fields
                            .into_iter()
                            .map(|(name, types)| (name.to_string(), types))
                            .collect(),
                    )
                } else {
                    VarType::Data(
                        fields
                            .into_iter()
                            .map(|(name, types)| (name.to_string(), types))
                            .collect(),
                    )
                }
            });

        // Struct
        let struct_field = just("(")
            .padded()
            .ignore_then(just(":"))
            .ignore_then(ident())
            .then(var_type_rec.clone().padded())
            .then_ignore(just(")"))
            .map(|(name, var_type): (String, VarType)| (name.to_string(), var_type));

        // Struct parser
        let struct_parser = just("(")
            .padded()
            .ignore_then(just("struct").padded())
            .ignore_then(generics().or_not())
            .then(
                struct_field
                    .clone()
                    .padded()
                    .repeated()
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(")"))
            .map(
                |(generic_types, fields): (Option<Vec<String>>, Vec<(String, VarType)>)| {
                    let mut field_names = std::collections::HashSet::new();
                    for (name, _) in &fields {
                        if !field_names.insert(name.clone()) {
                            panic!("Duplicate field name: {}", name);
                        }
                    }

                    if let Some(generics) = generic_types {
                        VarType::GenericStruct(
                            generics.iter().map(|s| s.to_string()).collect(),
                            fields,
                        )
                    } else {
                        VarType::Struct(fields)
                    }
                },
            );

        choice((
            basic_type,
            array_sized,
            array_unsized,
            generic_array_long_form,
            array_long_form,
            data,
            generic_ptr,
            ptr,
            tuple,
            fn_type,
            struct_parser,
            ident().map(VarType::IdentType),
        ))
        .padded()
    })
}

fn top_level_var_instruction<'a>() -> impl Parser<'a, &'a str, TopLevelDef> + Clone {
    let literal = literal().map(TopLevelDef::Literal);
    let typed = var_type().map(TopLevelDef::Typed);
    let function = function_statement().map(TopLevelDef::FnDef);

    choice((literal, typed, function))
}

fn def_statement<'a, T>(
    inner: impl Parser<'a, &'a str, T> + Clone,
) -> impl Parser<'a, &'a str, DefVar<T>> + Clone
where
    T: Clone,
{
    just("(")
        .ignore_then(just("def").padded())
        .ignore_then(ident().padded())
        .then(inner)
        .then_ignore(just(")"))
        .map(|(name, instruction): (String, T)| -> DefVar<T> {
            DefVar {
                name: name.to_string(),
                instruction,
            }
        })
}

fn function_parameters<'a>() -> impl Parser<'a, &'a str, (String, VarType)> + Clone {
    just("(")
        .padded()
        .ignore_then(just(":"))
        .ignore_then(ident())
        .then(var_type())
        .then_ignore(just(")"))
}

fn function_statement<'a>() -> impl Parser<'a, &'a str, FnDef> + Clone {
    type FnParseResult = (
        ((Option<Vec<String>>, Vec<(String, VarType)>), VarType),
        Statement,
    );

    just("(")
        .padded()
        .ignore_then(just("fn").padded())
        .ignore_then(generics().or_not())
        .then_ignore(just("[").padded())
        .then(function_parameters().repeated().collect::<Vec<_>>())
        .then_ignore(just("]"))
        .then(var_type())
        .then(statement())
        .then_ignore(just(")"))
        .map(
            |(((generic_types, params), return_type), statement): FnParseResult| -> FnDef {
                FnDef {
                    generic_types,
                    parameters: params,
                    return_type,
                    statement,
                }
            },
        )
}

pub fn statement<'a>() -> impl Parser<'a, &'a str, Statement> + Clone {
    recursive(|statement_rec| {
        let do_block = just("(")
            .padded()
            .ignore_then(just("do").padded())
            .ignore_then(
                statement_rec
                    .clone()
                    .padded()
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(")"))
            .map(Statement::DoBlock);

        // Call
        let call = just("(")
            .padded()
            .ignore_then(ident().padded())
            .then(
                statement_rec
                    .clone()
                    .padded()
                    .repeated()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(")"))
            .map(|(name, args)| Statement::Call(name, args));

        // If statement
        let if_statement = just("(")
            .padded()
            .ignore_then(just("if").padded())
            .ignore_then(statement_rec.clone().padded())
            .then(statement_rec.clone().padded())
            .then(statement_rec.clone().padded().or_not())
            .then_ignore(just(")").padded())
            .map(|((condition, if_block), else_block)| match else_block {
                None => Statement::If(Box::new(condition), Box::new(if_block)),
                Some(else_block) => Statement::IfElse(
                    Box::new(condition),
                    Box::new(if_block),
                    Box::new(else_block),
                ),
            });

        let def = def_statement(statement_rec.clone()).map(|def| Statement::DefVar(def.boxed()));

        choice((
            if_statement,
            do_block,
            def,
            call,
            ident().map(Statement::Ident),
            literal().map(Statement::Literal),
        ))
        .padded()
    })
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<TopLevelStatement>> {
    let use_statement = just("(")
        .ignore_then(just("use").padded())
        .ignore_then(just(":header").padded().or_not().map(|o| o.is_some()))
        .then(
            just("\"")
                .ignore_then(none_of("\"").repeated().collect::<String>())
                .then_ignore(just("\"")),
        )
        .then_ignore(just(")"));

    let def_use = just("(")
        .ignore_then(just("def").padded())
        .ignore_then(ident().padded())
        .then(use_statement)
        .then_ignore(just(")"))
        .map(|(name, (is_header, import)): (String, (bool, String))| {
            if is_header {
                TopLevelStatement::UseHeader(name.to_string(), import)
            } else {
                TopLevelStatement::Use(name.to_string(), import)
            }
        });

    let def_var = def_statement(top_level_var_instruction()).map(TopLevelStatement::TopLevelDef);

    let def = def_var.or(def_use);

    let type_alias = just("(")
        .ignore_then(just("type").padded())
        .ignore_then(ident().padded())
        .then(var_type())
        .then_ignore(just(")"))
        .map(|(name, var_type): (String, VarType)| {
            TopLevelStatement::TypeAlias(name.to_string(), var_type)
        });

    choice((def, type_alias))
        .padded()
        .repeated()
        .collect::<Vec<_>>()
}
