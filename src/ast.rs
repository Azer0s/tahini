#[derive(Clone, Debug, PartialEq)]
pub struct FnDef {
    pub generic_types: Option<Vec<String>>,
    pub parameters: Vec<(String, VarType)>,
    pub return_type: VarType,
    pub statement: Statement,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefVar<T>
where
    T: Clone,
{
    pub name: String,
    pub instruction: T,
}

impl<T> DefVar<T>
where
    T: Clone,
{
    pub fn boxed(self) -> DefVar<Box<T>> {
        DefVar {
            name: self.name,
            instruction: Box::new(self.instruction),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Ident(String),
    Literal(Literal),
    DoBlock(Vec<Statement>),
    Call(String, Vec<Statement>),
    DefVar(DefVar<Box<Statement>>),
    If(Box<Statement>, Box<Statement>),
    IfElse(Box<Statement>, Box<Statement>, Box<Statement>),
    For(Box<Statement>, Box<Statement>),
    ForRange(String, Box<Statement>, Box<Statement>),
    GenericCall(String, Vec<String>, Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
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
    Data(Vec<(String, Vec<VarType>)>),
    GenericData(Vec<String>, Vec<(String, Vec<VarType>)>),
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
    Tuple(Vec<Statement>),
    Data(String, Vec<Statement>),
    Array(Vec<Statement>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelDef {
    Literal(Literal),
    Typed(VarType),
    FnDef(FnDef),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelStatement {
    TypeAlias(String, VarType),
    TopLevelDef(DefVar<TopLevelDef>),
    Use(String, String),
    UseHeader(String, String),
    ExportAll(),
    Export(Vec<String>),
}
