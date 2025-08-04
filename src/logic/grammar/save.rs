use std::path::Path;
use super::{Grammar, Symbol};
use super::typing::{Premise, TypingJudgment};


impl Grammar {
    /// Produce the textual specification string.
    pub fn to_spec_string(&self) -> String {
        let mut out = String::new();
        let mut nt_list: Vec<_> = self.productions.keys().cloned().collect();
        nt_list.sort();

        // ---------- Productions ----------
        out.push_str("// --- Production Rules ---\n");
        for nt in nt_list {
            if let Some(alts) = self.productions.get(&nt) {
                let mut first = true;
                for prod in alts {
                    let lhs = if let Some(rule_name) = &prod.rule {
                        format!("{}({})", nt, rule_name)
                    } else {
                        nt.clone()
                    };
                    
                    let rhs = format_rhs(&prod.rhs);
                    
                    if first {
                        out.push_str(&format!("{} ::= {}", lhs, rhs));
                        first = false;
                    } else {
                        out.push_str(&format!(" | {}", rhs));
                    }
                }
                out.push('\n');
            }
        }
        out.push_str("\n");

        // ---------- Typing rules ----------
        if !self.typing_rules.is_empty() {
            out.push_str("// --- Typing Rules ---\n");
            let mut rule_list: Vec<_> = self.typing_rules.values().collect();
            rule_list.sort_by_key(|r| &r.name);
            
            for rule in rule_list {
                out.push_str(&format_premises(&rule.premises));
                out.push('\n');
                let concl_str = format_conclusion(&rule.conclusion);
                let line = "-".repeat(std::cmp::max(20, concl_str.len() + 5));
                out.push_str(&format!("{} ({})\n", line, rule.name));
                out.push_str(&concl_str);
                out.push_str("\n\n");
            }
        }

        out
    }

    /// Write the textual specification to a file on disk.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path, self.to_spec_string())
    }
}

/// Helper to format the right-hand side of a production
fn format_rhs(rhs_symbols: &[Symbol]) -> String {
    rhs_symbols.iter().map(|symbol| {
        if let Some(binding) = &symbol.binding {
            format!("{}[{}]", symbol.value, binding)
        } else {
            // Handle special tokens that need quotes
            if symbol.value.len() > 1 && (symbol.value.starts_with('\'') || symbol.value.starts_with('"')) {
                 symbol.value.clone()
            } else if !symbol.value.chars().all(char::is_alphanumeric) {
                 // Do not quote regex patterns that start with '/'
                 if symbol.value.starts_with('/') && symbol.value.ends_with('/') {
                     symbol.value.clone()
                 } else {
                     format!("'{}'", symbol.value)
                 }
            } else {
                 symbol.value.clone()
            }
        }
    }).collect::<Vec<_>>().join(" ")
}

/// Helper to format a list of premises as a string
fn format_premises(premises: &[super::typing::Premise]) -> String {
    premises.iter().map(|p| match p {
        Premise::Judgment(TypingJudgment { extensions, expression, type_expr }) => {
            let ctx = if extensions.is_empty() {
                "Γ".to_string()
            } else {
                let exts = extensions.iter().map(|e| format!("{}:{}", e.variable, e.type_expr)).collect::<Vec<_>>().join(",");
                format!("Γ,{}", exts)
            };
            format!("{} ⊢ {} : {}", ctx, expression, type_expr)
        }
        Premise::Membership { variable, context } => format!("{} ∈ {}", variable, context),
        Premise::TypeRelation { left_type, right_type, relation } => format!("{} {} {}", left_type, relation, right_type),
        Premise::Compound(inner) => format_premises(inner),
    }).collect::<Vec<_>>().join(", ")
}


fn format_conclusion(conclusion: &super::typing::Conclusion) -> String {
    use super::typing::Conclusion;
    match conclusion {
        Conclusion::TypeValue(s) => s.clone(),
        Conclusion::Judgment(j) => {
            let ctx = if j.extensions.is_empty() {
                "Γ".to_string()
            } else {
                let exts = j.extensions.iter().map(|e| format!("{}:{}", e.variable, e.type_expr)).collect::<Vec<_>>().join(",");
                format!("Γ,{}", exts)
            };
            format!("{} ⊢ {} : {}", ctx, j.expression, j.type_expr)
        }
        Conclusion::ContextLookup(var) => format!("Γ({})", var),
    }
}

