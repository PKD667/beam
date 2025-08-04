use crate::logic::ast::ASTNode;

/// Deep AST equality check for testing
pub fn ast_eq(a: &ASTNode, b: &ASTNode) -> bool {
    if a.kind != b.kind || a.value != b.value || a.binding != b.binding {
        return false;
    }
    match (&a.children, &b.children) {
        (Some(ac), Some(bc)) => {
            if ac.len() != bc.len() { return false; }
            for (x, y) in ac.iter().zip(bc.iter()) {
                if !ast_eq(x, y) { return false; }
            }
            true
        }
        (None, None) => true,
        _ => false,
    }
}
