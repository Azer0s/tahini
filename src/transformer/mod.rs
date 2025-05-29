pub mod ast;
pub mod dollar;
mod dollar_test;

use crate::ast as orig;

pub fn transform(module: &[orig::TopLevelStatement]) -> ast::Module {
    ast::Module {
        items: module.to_vec(),
    }
}
