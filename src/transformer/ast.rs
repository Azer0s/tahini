#[derive(Clone, Debug, PartialEq)]
pub enum AccessSegment {
    Field(String),
    Index(Box<crate::ast::Statement>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransformedStmt {
    // Wrapper around original statement (for constructs untouched by this transformer)
    Orig(crate::ast::Statement),

    // Chained access like `a->b->c` or `a[0][i]`
    ChainAccess {
        root: Box<crate::ast::Statement>,
        segments: Vec<AccessSegment>,
    },

    // Chained assignment like `a->b->c = value`
    ChainAssign {
        root: Box<crate::ast::Statement>,
        segments: Vec<AccessSegment>,
        value: Box<crate::ast::Statement>,
    },

    // Closure creation after hoisting lambdas
    MakeClosure {
        fn_name: String,
        env: Vec<crate::ast::Statement>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub items: Vec<crate::ast::TopLevelStatement>,
}
