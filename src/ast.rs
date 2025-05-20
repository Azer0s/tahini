use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VarType {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float16,
    Float32,
    Float64,
    Float128,
    Bool,
    Void,
    IdentType(String),
    ArraySized(Box<VarType>, usize),
    ArrayUnsized(Box<VarType>),
    GenericArraySized(String, usize),
    GenericArrayUnsized(String),
    Ptr(Box<VarType>),
    GenericPtr(String),
    Tuple(Vec<VarType>),
    GenericTuple(Vec<String>, Vec<VarType>),
    Struct(Vec<(String, VarType)>),
    GenericStruct(Vec<String>, Vec<(String, VarType)>),
    Fn(Vec<VarType>, Box<VarType>),
    FnWithVarArgs(Vec<VarType>, Box<VarType>),
    GenericFn(Vec<String>, Vec<VarType>, Box<VarType>),
    GenericFnWithVarArgs(Vec<String>, Vec<VarType>, Box<VarType>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Atom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Literal(Literal),
    Var(String),
    Call(String, Vec<Instruction>),
    If(Box<Instruction>, Box<Instruction>, Option<Box<Instruction>>),
    For(String, Box<Instruction>, Box<Instruction>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarInstruction {
    Literal(Literal),
    TypedInit(VarType, Box<Instruction>),
    Typed(VarType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelStatement {
    TypeAlias(String, VarType),
    DefVar(String, VarInstruction),
    Use(String, String),
    UseHeader(String, String),
}
