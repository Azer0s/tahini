use chumsky::prelude::*;
use chumsky::text::ident;
use crate::ast::{VarType, Literal, VarInstruction, TopLevelStatement};

pub fn literal<'a>() -> impl Parser<'a, &'a str, Literal> + Clone {
    let int = text::int(10)
        .map(|s: &str| Literal::Int(s.parse().unwrap()));

    let float = text::int(10)
        .then_ignore(just("."))
        .then(text::int(10))
        .map(|(int_part, frac_part): (&'a str, &'a str)| {
            let float_str = format!("{}.{}", int_part, frac_part);
            Literal::Float(float_str.parse().unwrap())
        });

    let atom = just(":")
        .ignore_then(text::unicode::ident())
        .map(|s: &'a str| Literal::Atom(s.to_string()));

    let bool = just("true").or(just("false"))
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
            just("void").to(VarType::Void)
        ));

        // Array
        let array_sized = just("[")
            .padded()
            .ignore_then(var_type_rec.clone().padded())
            .then_ignore(just(",").padded())
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
            .map(|var_type| {
                VarType::ArrayUnsized(Box::from(var_type))
            });

        // Pointer
        let ptr = just("(")
            .padded()
            .ignore_then(just("ptr").padded())
            .ignore_then(var_type_rec.clone().padded())
            .then_ignore(just(")"))
            .map(|var_type| VarType::Ptr(Box::from(var_type)));

        // Tuple
        let tuple_shorthand = just("{")
            .padded()
            .ignore_then(var_type_rec.clone().padded().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just("}"))
            .map(VarType::Tuple);

        let tuple_longform = just("(")
            .padded()
            .ignore_then(just("tuple").padded())
            .ignore_then(var_type_rec.clone().padded().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just(")"))
            .map(VarType::Tuple);

        let tuple = tuple_shorthand.or(tuple_longform);

        // Function
        let fn_type = just("fn")
            .padded()
            .ignore_then(
                just("[")
                .padded()
                .ignore_then(var_type_rec.clone().padded().repeated().collect::<Vec<_>>())
                    .then(just("...").padded().or_not().map(|va| va.is_some()))
                .then_ignore(just("]").padded())
            )
            .then(var_type_rec.clone())
            .map(|((args, is_va), ret)| {
                if is_va {
                    VarType::FnWithVarArgs(args, Box::new(ret))
                } else {
                    VarType::Fn(args, Box::new(ret))
                }
            });

        // Struct
        let struct_field = just("(")
            .padded()
            .ignore_then(just(":").padded())
            .ignore_then(ident())
            .then(var_type_rec.clone().padded())
            .then_ignore(just(")"))
            .map(|(name, var_type): (&'a str, VarType)| {
                (name.to_string(), var_type)
            });

        // Generic struct parser
        let generic_struct = just("(")
            .padded()
            .ignore_then(just("struct").padded())
            .ignore_then(
                just("<")
                .then(
                    text::ident()
                    .padded()
                    .separated_by(just(" "))
                    .at_least(1)
                    .collect::<Vec<_>>()
                )
                .then_ignore(just(">"))
            )
            .then(struct_field.clone().padded().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just(")"))
            .map(|((_, generic_types), fields): ((_, Vec<&'a str>), Vec<(String, VarType)>)| {
                let mut field_names = std::collections::HashSet::new();
                for (name, _) in &fields {
                    if !field_names.insert(name.clone()) {
                        panic!("Duplicate field name: {}", name);
                    }
                }
                
                VarType::GenericStruct(
                    generic_types.into_iter().map(|s| s.to_string()).collect(),
                    fields
                )
            });

        // Regular struct parser
        let regular_struct = just("(")
            .padded()
            .ignore_then(just("struct").padded())
            .ignore_then(struct_field.clone().padded().repeated().at_least(1).collect::<Vec<_>>())
            .then_ignore(just(")"))
            .map(|fields: Vec<(String, VarType)>| {
                let mut field_names = std::collections::HashSet::new();
                for (name, _) in &fields {
                    if !field_names.insert(name.clone()) {
                        panic!("Duplicate field name: {}", name);
                    }
                }
                
                VarType::Struct(fields)
            });

        let struct_parser = generic_struct.or(regular_struct);

        choice((
            basic_type,
            array_sized,
            array_unsized,
            ptr,
            tuple,
            fn_type,
            struct_parser,
            ident().map(|s: &'a str| VarType::IdentType(s.to_string()))
        )).padded()
    })
}

fn var_instruction<'a>() -> impl Parser<'a, &'a str, VarInstruction> + Clone {
    let literal = literal().map(VarInstruction::Literal);
    let typed = var_type().map(VarInstruction::Typed);

    literal.or(typed)
}

pub fn parser<'a>() -> impl Parser<'a, &'a str, Vec<TopLevelStatement>> {
    let ident = ident().padded();
    
    let use_statement = just("(")
        .ignore_then(just("use").padded())
        .ignore_then(just(":header").padded().or_not().map(|o| o.is_some()))
        .then(just("\"")
            .ignore_then(none_of("\"").repeated().collect::<String>())
            .then_ignore(just("\"")))
        .then_ignore(just(")"));

    let def_use = just("(")
        .ignore_then(just("def").padded())
        .ignore_then(ident)
        .then(use_statement)
        .then_ignore(just(")"))
        .map(|(name, (is_header, import)): (&'a str, (bool, String))| {
            if is_header {
                TopLevelStatement::UseHeader(name.to_string(), import)
            } else {
                TopLevelStatement::Use(name.to_string(), import)
            }
        });
    
    let def_var = just("(")
        .ignore_then(just("def").padded())
        .ignore_then(ident)
        .then(var_instruction())
        .then_ignore(just(")"))
        .map(|(name, var_instruction): (&'a str, VarInstruction)| {
            TopLevelStatement::DefVar(name.to_string(), var_instruction)
        });
    
    let def = def_var.or(def_use);

    let type_alias = just("(")
        .ignore_then(just("type").padded())
        .ignore_then(ident)
        .then(var_type())
        .then_ignore(just(")"))
        .map(|(name, var_type): (&'a str, VarType)| {
            TopLevelStatement::TypeAlias(name.to_string(), var_type)
        });

    choice((def, type_alias))
        .padded()
        .repeated()
        .collect::<Vec<_>>()
}
