use super::ast::AccessSegment;
use crate::ast::Statement;

#[derive(Clone, Debug, PartialEq)]
pub enum DollarChain {
    Access {
        root: Box<Statement>,
        segments: Vec<AccessSegment>,
    },
    Assign {
        root: Box<Statement>,
        segments: Vec<AccessSegment>,
        value: Box<Statement>,
    },
}

/// Attempt to recognise a chain of nested `$` accesses/assignments and linearise them.
/// Returns `None` if the provided statement is **not** rooted at a `$` operator.
pub fn try_linearize(stmt: &Statement) -> Option<DollarChain> {
    match stmt {
        Statement::GetField(field, target) => {
            let mut segments = Vec::new();
            segments.push(AccessSegment::Field(field.clone()));
            let (root, rest) = collect_segments(target);
            segments.extend(rest);
            Some(DollarChain::Access {
                root: Box::new(root),
                segments,
            })
        }
        Statement::GetIndexed(index, target) => {
            let mut segments = Vec::new();
            segments.push(AccessSegment::Index(index.clone()));
            let (root, rest) = collect_segments(target);
            segments.extend(rest);
            Some(DollarChain::Access {
                root: Box::new(root),
                segments,
            })
        }
        Statement::SetField(field, target, value) => {
            let mut segments = Vec::new();
            segments.push(AccessSegment::Field(field.clone()));
            let (root, rest) = collect_segments(target);
            segments.extend(rest);
            Some(DollarChain::Assign {
                root: Box::new(root),
                segments,
                value: value.clone(),
            })
        }
        Statement::SetIndexed(index, target, value) => {
            let mut segments = Vec::new();
            segments.push(AccessSegment::Index(index.clone()));
            let (root, rest) = collect_segments(target);
            segments.extend(rest);
            Some(DollarChain::Assign {
                root: Box::new(root),
                segments,
                value: value.clone(),
            })
        }
        _ => None,
    }
}

/// Helper to recursively collect segments from nested `$` operator statements.
/// Returns the base/root expression (that is **not** another `$` op) and the
/// segments collected **in reverse order** (closest segment first).
fn collect_segments(stmt: &Statement) -> (Statement, Vec<AccessSegment>) {
    match stmt {
        Statement::GetField(field, inner) => {
            let mut segs = vec![AccessSegment::Field(field.clone())];
            let (root, rest) = collect_segments(inner);
            segs.extend(rest);
            (root, segs)
        }
        Statement::GetIndexed(index, inner) => {
            let mut segs = vec![AccessSegment::Index(index.clone())];
            let (root, rest) = collect_segments(inner);
            segs.extend(rest);
            (root, segs)
        }
        Statement::SetField(..) | Statement::SetIndexed(..) => {
            // We should never see assignment in nested positions for a valid `$` chain.
            // Treat as root for now.
            (stmt.clone(), Vec::new())
        }
        _ => (stmt.clone(), Vec::new()),
    }
}
