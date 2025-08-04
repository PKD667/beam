pub mod utils;
pub mod load;
pub mod save;
pub mod typing;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Symbol {
    pub value: String,
    pub binding: Option<String>,
}

impl Symbol {
    pub fn new(value: String) -> Self {
        Symbol { value, binding: None }
    }
    
    pub fn with_binding(value: String, binding: String) -> Self {
        Symbol { value, binding: Some(binding) }
    }
}

/// Convenience alias for non-terminal symbols.
pub type Nonterminal = String;
/// Convenience alias for terminal symbols.
pub type Terminal = String;
/// A single production rule `left ::= right₀ right₁ …`.
#[derive(Debug, Clone, PartialEq)]
pub struct Production {
    pub rule: Option<String>,
    pub rhs: Vec<Symbol>,
}

use typing::TypingRule;

/// A complete grammar consisting of context-free productions and
/// inference-style typing rules.
#[derive(Debug, Default, PartialEq)]
pub struct Grammar {
    pub productions: HashMap<Nonterminal, Vec<Production>>,
    pub typing_rules: HashMap<String, TypingRule>, // name -> rule
    pub special_tokens: Vec<String>,
}

impl Grammar {
    /// Create an empty grammar.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a special token to the grammar if not already present.
    pub fn add_special_token(&mut self, token: String) {
        if !self.special_tokens.contains(&token) {
            self.special_tokens.push(token);
        }
    }

    /// Add a typing rule to the grammar.
    pub fn add_typing_rule(&mut self, rule: TypingRule) {
        self.typing_rules.insert(rule.name.clone(), rule);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use super::typing::Conclusion;

    pub(crate) const STLC_SPEC: &str = r#"

    // Identifier
    Identifier ::= /[a-zA-Z][a-zA-Z0-9_]*/

    // Variables
    Variable(var) ::= Identifier[x]

    // Type names  
    TypeName ::= Identifier

    // Base types
    BaseType ::= TypeName | '(' Type ')'

    // Function types (right-associative)
    Type ::= BaseType[τ₁] '->' Type[τ₂] | BaseType[τ]

    // Typed parameter
    TypedParam ::= Variable[x] ':' Type[τ]

    // Lambda abstraction
    Lambda(lambda) ::= 'λ' TypedParam '.' Term[e]

    // Base terms (cannot be applications)
    BaseTerm ::= Variable | Lambda | '(' Term ')'

    // Applications (left-associative via iteration)
    Application(app) ::= BaseTerm[f] BaseTerm[e]

    // Terms
    Term ::= Application[e] | BaseTerm[e]

    // Typing Rules
    x ∈ Γ
    ------------ (var)
    Γ(x)

    Γ[x:τ₁] ⊢ e : τ₂
    --------------------------- (lambda)
    τ₁ → τ₂

    Γ ⊢ f : τ₁ → τ₂, Γ ⊢ e : τ₁
    -------------------------------- (app)
    τ₂
    "#;

    #[test]
    fn parses_unified_stlc_grammar() {
        let grammar = Grammar::load(STLC_SPEC).expect("Parser failed");

        // Check productions
        assert!(grammar.productions.contains_key("Variable"));
        assert!(grammar.productions.contains_key("Lambda"));
        let lambda_prods = grammar.productions.get("Lambda").unwrap();
        assert_eq!(lambda_prods.len(), 1);
        assert_eq!(lambda_prods[0].rule, Some("lambda".to_string()));
        
        let typed_param_prod = grammar.productions.get("TypedParam").unwrap();
        let var_symbol = typed_param_prod[0].rhs.iter().find(|s| s.value == "Variable").unwrap();
        assert_eq!(var_symbol.binding, Some("x".to_string()));

        // Check typing rules
        assert_eq!(grammar.typing_rules.len(), 3);
        assert!(grammar.typing_rules.contains_key("var"));
        assert!(grammar.typing_rules.contains_key("lambda"));
        assert!(grammar.typing_rules.contains_key("app"));
        
        let lambda_rule = grammar.typing_rules.get("lambda").unwrap();
        match &lambda_rule.conclusion {
            Conclusion::TypeValue(s) => assert_eq!(s, "τ₁ → τ₂"),
            _ => panic!("Expected TypeValue for lambda conclusion"),
        }
        assert_eq!(lambda_rule.premises.len(), 1);
        match &lambda_rule.premises[0] {
            typing::Premise::Judgment(typing::TypingJudgment { extensions, expression, type_expr }) => {
                assert_eq!(extensions.len(), 1);
                assert_eq!(extensions[0].variable, "x");
                assert_eq!(extensions[0].type_expr, "τ₁");
                assert_eq!(expression, "e");
                assert_eq!(type_expr, "τ₂");
            }
            _ => panic!("Expected typing judgment for lambda premise"),
        }
        
        let app_rule = grammar.typing_rules.get("app").unwrap();
        match &app_rule.conclusion {
            Conclusion::TypeValue(s) => assert_eq!(s, "τ₂"),
            _ => panic!("Expected TypeValue for app conclusion"),
        }
        assert_eq!(app_rule.premises.len(), 2);
        match &app_rule.premises[0] {
            typing::Premise::Judgment(typing::TypingJudgment { extensions, expression, type_expr }) => {
                assert!(extensions.is_empty());
                assert_eq!(expression, "f");
                assert_eq!(type_expr, "τ₁ → τ₂");
            }
            _ => panic!("Expected typing judgment for app premise 0"),
        }
        match &app_rule.premises[1] {
            typing::Premise::Judgment(typing::TypingJudgment { extensions, expression, type_expr }) => {
                assert!(extensions.is_empty());
                assert_eq!(expression, "e");
                assert_eq!(type_expr, "τ₁");
            }
            _ => panic!("Expected typing judgment for app premise 1"),
        }
    }

    #[test]
    fn roundtrip_write_and_parse() {
        let grammar1 = Grammar::load(STLC_SPEC).expect("parse");
        let spec = grammar1.to_spec_string();
        println!("Generated spec:\n{}", spec);
        let grammar2 = Grammar::load(&spec).expect("re-parse");

        // Compare essential parts instead of direct equality (HashMap ordering can differ)
        assert_eq!(grammar1.productions.len(), grammar2.productions.len());
        assert_eq!(grammar1.typing_rules.len(), grammar2.typing_rules.len());
        
        // Check that all production keys exist in both
        for key in grammar1.productions.keys() {
            assert!(grammar2.productions.contains_key(key), "Missing production key: {}", key);
            assert_eq!(grammar1.productions[key], grammar2.productions[key], "Production mismatch for key: {}", key);
        }
        
        // Check that all typing rule keys exist in both
        for key in grammar1.typing_rules.keys() {
            assert!(grammar2.typing_rules.contains_key(key), "Missing typing rule key: {}", key);
            assert_eq!(grammar1.typing_rules[key], grammar2.typing_rules[key], "Typing rule mismatch for key: {}", key);
        }
        
        // Special tokens should be the same (order doesn't matter)
        let mut tokens1 = grammar1.special_tokens.clone();
        let mut tokens2 = grammar2.special_tokens.clone();
        tokens1.sort();
        tokens2.sort();
        assert_eq!(tokens1, tokens2);
    }
}
