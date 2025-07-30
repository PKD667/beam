use crate::logic::ast::{ASTNode, NodeKind};
use crate::logic::grammar::TypingRule;
use std::collections::HashMap;
use regex::Regex;

pub mod validation;
use validation::*;
pub mod parse;
use parse::*;

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

#[derive(Debug, Clone, PartialEq)]
pub struct TypingExtension {
    pub type_expr: String,
    pub variable: String,
}

/// Represents different types of premises in deterministic format
#[derive(Debug, Clone, PartialEq)]
pub enum Premise {
    /// Typing judgment: Γ ⊢ e : τ
    TypingJudgment {
        extensions: Vec<TypingExtension>,
        expression: String,
        type_expr: String,
    },
    /// Context membership: x ∈ Γ
    Membership {
        variable: String,
        context: String,
    },
    /// Type relation: τ₁ = τ₂, τ₁ <: τ₂
    TypeRelation {
        left_type: String,
        right_type: String,
        relation: String,
    },
    /// Compound premises (e.g., "Γ ⊢ f : τ -> σ, Γ ⊢ e : τ")
    Compound(Vec<Premise>),
}

/// A deterministic, validated typing rule
#[derive(Debug, Clone)]
pub struct ParsedTypingRule {
    pub name: String,
    pub premises: Vec<Premise>,
    pub conclusion: String,
}

impl ParsedTypingRule {
    /// Parse and validate a typing rule in deterministic format
    pub fn new(rule: &TypingRule) -> Result<Self, String> {

        let premises = rule.premises
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|premise_str| {
                let premise = Self::parse_premise(premise_str)?;
                Ok(premise)
            })
            .collect::<Result<Vec<_>, String>>()?;

        // Validate conclusion
        validate_conclusion(&rule.conclusion)?;
        
        Ok(ParsedTypingRule {
            name: rule.name.clone(),
            premises,
            conclusion: rule.conclusion.clone(),
        })
    }



    /// Parse and validate a typing rule in deterministic format
    fn parse_premise(premise_str: &str) -> Result<Premise, String> {
        let premise_str = premise_str.trim();

        // Case 1: Starts with context (e.g., "Γ ⊢ e : τ")
        if premise_str.starts_with("Γ") && premise_str.contains('⊢') {
            // Parse typing judgment
            // Example: "Γ ⊢ e : τ"
            let (vec_extensions, expression, type_expr) = parse_judgement(premise_str)?;

            let extensions = match vec_extensions {
                Some(exts) => exts.into_iter()
                    .map(|(var, ty)| TypingExtension { variable: var, type_expr: ty })
                    .collect(),
                None => Vec::new(),
            };

            return Ok(Premise::TypingJudgment {
                extensions,
                expression,
                type_expr,
            });
        }
        // Case 2: Has context and includes one relation symbol (e.g., "x ∈ Γ")
        else if premise_str.contains('∈') {
            let (var, ctx) = parse_membership(premise_str)?;
            return Ok(Premise::Membership {
                variable: var,
                context: ctx,
            });
        }
        // Case 3: Has relation symbols (e.g., "τ₁ = τ₂", "τ₁ <: τ₂")
        else if RELATION_SYMBOLS.iter().any(|&sym| premise_str.contains(sym)) {
            let (left, right, relation) = parse_type_relation(premise_str)?;
            return Ok(Premise::TypeRelation {
                left_type: left,
                right_type: right,
                relation,
            });
        }
        Err(format!("Unknown premise format: {}", premise_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_typing_rule() {
        let rule = TypingRule {
            name: "ExampleRule".to_string(),
            premises: "Γ ⊢ e : τ, x ∈ Γ, τ₁ <: τ₂".to_string(),
            conclusion: "Γ ⊢ e : σ".to_string(),
        };
        let parsed_rule = ParsedTypingRule::new(&rule).unwrap();
        assert_eq!(parsed_rule.name, "ExampleRule");
        assert_eq!(parsed_rule.premises.len(), 3);
        match &parsed_rule.premises[0] {    
            Premise::TypingJudgment { expression, type_expr, .. } => {
                assert_eq!(expression, "e");
                assert_eq!(type_expr, "τ");
            }
            _ => panic!("Expected typing judgment"),
        }
        match &parsed_rule.premises[1] {
            Premise::Membership { variable, context } => {
                assert_eq!(variable, "x");
                assert_eq!(context, "Γ");
            }
            _ => panic!("Expected membership"),
        }
        match &parsed_rule.premises[2] {
            Premise::TypeRelation { left_type, right_type, relation } => {
                assert_eq!(left_type, "τ₁");
                assert_eq!(right_type, "τ₂");
                assert_eq!(relation, "<:");
            }
            _ => panic!("Expected type relation"),
        }
        assert_eq!(parsed_rule.conclusion, "Γ ⊢ e : σ");
    }
    #[test]
    fn test_parse_context_extensions() {
        use crate::logic::check::parse::parse_context_extensions;
        // Single extension
        let (base, exts) = parse_context_extensions("Γ,x:τ").unwrap();
        assert_eq!(base, "Γ");
        assert_eq!(exts, vec![("x".to_string(), "τ".to_string())]);
        // Multiple extensions
        let (base, exts) = parse_context_extensions("Γ,x:τ₁,y:τ₂").unwrap();
        assert_eq!(base, "Γ");
        assert_eq!(exts, vec![
            ("x".to_string(), "τ₁".to_string()),
            ("y".to_string(), "τ₂".to_string()),
        ]);
        // No extensions
        let (base, exts) = parse_context_extensions("Γ").unwrap();
        assert_eq!(base, "Γ");
        assert!(exts.is_empty());
        // Invalid: missing Γ
        assert!(parse_context_extensions("x:τ").is_err());
        // Invalid: malformed extension
        assert!(parse_context_extensions("Γ,xτ").is_err());
    }
}