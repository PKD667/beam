use super::utils::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TypingExtension {
    pub type_expr: String,
    pub variable: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypingJudgment {
    pub extensions: Vec<TypingExtension>,
    pub expression: String,
    pub type_expr: String,
}

/// Represents different types of premises in deterministic format
#[derive(Debug, Clone, PartialEq)]
pub enum Premise {
    /// Typing judgment: Γ ⊢ e : τ
    Judgment(TypingJudgment),
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

#[derive(Debug, Clone, PartialEq)]
pub enum Conclusion {
    TypeValue(String), // value of a type expression (e.g., "τ₁ → τ₂")
    Judgment(TypingJudgment), 
    ContextLookup(String), // value of var to be looked up in context
}

/// A typing rule written in standard inference-rule notation.
#[derive(Debug, Clone, PartialEq)]
pub struct TypingRule {
    pub name: String,
    pub premises: Vec<Premise>,
    pub conclusion: Conclusion,
}

impl TypingRule {
    /// Parse and validate a typing rule in deterministic format
    pub fn new(str_premises: String, conclusion: String, name: String) -> Result<Self, String> {
        println!("Creating TypingRule with premises: {}, conclusion: {}, name: {}", str_premises, conclusion, name);
        let premises = str_premises
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|premise_str| {
                let premise = Self::parse_premise(premise_str)?;
                Ok(premise)
            })
            .collect::<Result<Vec<_>, String>>()?;

        let conclusion = Self::parse_conclusion(&conclusion)?;
        
        Ok(Self {
            name: name.clone(),
            premises,
            conclusion,
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

            return Ok(Premise::Judgment(
                TypingJudgment {
                    extensions,
                    expression,
                    type_expr,
                }
            ));
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

    fn parse_conclusion(conclusion_str: &str) -> Result<Conclusion, String> {
        let conclusion_str = conclusion_str.trim();
        if conclusion_str.starts_with("Γ(") {
            // Context lookup format: "Γ(x)"
            let var = conclusion_str.trim_start_matches("Γ(").trim_end_matches(')');
            Ok(Conclusion::ContextLookup(var.to_string()))
        } 
        else if conclusion_str.contains('⊢') {
            // Typing judgment format
            let (extensions, expression, type_expr) = parse_judgement(conclusion_str)?;
            let extensions = extensions.unwrap_or_default();
            Ok(Conclusion::Judgment(TypingJudgment {
                extensions: extensions.into_iter()
                    .map(|(var, ty)| TypingExtension { variable: var, type_expr: ty })
                    .collect(),
                expression,
                type_expr,
            }))
        } else if validate_type_expr(conclusion_str) {
            // Type expression format
            Ok(Conclusion::TypeValue(conclusion_str.to_string()))
        } else {
            Err(format!("Invalid conclusion format: {}", conclusion_str))
        }
    }

}

///---------------
/// Type Validation
///---------------


const TYPE_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_λτ→₁₂₃₄₅₆₇₈₉₀ ";


pub fn validate_type_expr(expr: &str) -> bool {
    // A very basic check for valid type expressions
    // This can be expanded with more complex rules as needed
    !expr.is_empty() && expr.chars().all(|c| TYPE_CHARS.contains(c))
}
