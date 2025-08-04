use crate::logic::ast::{ASTNode, NodeKind};
use crate::logic::grammar::typing::TypingRule;
use std::collections::HashMap;
use regex::Regex;

/// Represents a typing context that maps variables to their types
#[derive(Debug, Clone, PartialEq)]
pub struct TypingContext {
    /// Maps variable names to their types
    pub bindings: HashMap<String, String>,
}

impl TypingContext {
    /// Create a new empty typing context
    pub fn new() -> Self {
        TypingContext {
            bindings: HashMap::new(),
        }
    }   
}