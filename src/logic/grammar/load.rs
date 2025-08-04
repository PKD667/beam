use crate::logic::grammar::{utils::parse_inference_rule, Grammar, Production, TypingRule};
use super::utils::{parse_nonterminal, parse_production, parse_rhs, special_tokens};
use regex::Regex;

impl Grammar {
    /// Parse the textual specification into a `Grammar`.
    pub fn load(input: &str) -> Result<Grammar, String> {
        let mut grammar = Grammar::new();
        
        // Split input into blocks separated by blank lines
        let blocks: Vec<&str> = input.split("\n\n").filter(|b| !b.trim().is_empty()).collect();
        
        for block in blocks {
            let lines: Vec<&str> = block
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with("//"))
                .collect();
                
            if lines.is_empty() {
                continue;
            }
            
            // Check if this block contains a production rule
            if lines.iter().any(|line| line.contains("::=")) {
                // Production block - may contain multiple productions
                let mut i = 0;
                while i < lines.len() {
                    let line = lines[i];
                    if line.contains("::=") {
                        // Start of a new production
                        let mut production_lines = vec![line];
                        i += 1;
                        
                        // Collect any continuation lines starting with |
                        while i < lines.len() && lines[i].starts_with('|') {
                            production_lines.push(lines[i]);
                            i += 1;
                        }
                        
                        // Parse this production
                        let production_str = production_lines.join(" ");
                        let (lhs_str, rhs_str) = parse_production(&production_str)?;
                        let (name, rule_name) = parse_nonterminal(&lhs_str)?;
                        let rhs_alternatives = parse_rhs(&rhs_str)?;
                        
                        // Extract special tokens
                        let new_specials = special_tokens(&rhs_str);
                        for sym in new_specials {
                            grammar.add_special_token(sym);
                        }
                        
                        // Create productions
                        for alt_symbols in rhs_alternatives {
                            let production = Production {
                                rule: rule_name.clone(),
                                rhs: alt_symbols,
                            };
                            grammar.productions.entry(name.clone()).or_default().push(production);
                        }
                    } else {
                        i += 1;
                    }
                }
            } else {
                let ( premises, conclusion,name) = parse_inference_rule(&lines)?;
                grammar.add_typing_rule(TypingRule::new(premises, conclusion, name)?);
            }

        }
        
        Ok(grammar)
    }
}

