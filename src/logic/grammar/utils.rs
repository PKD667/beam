use regex::Regex;
use super::{TypingRule, Symbol};

// collection of utils for working with grammar definitions
pub fn is_regex(pattern: &str) -> bool {
    // Only slash-delimited patterns: /regex/
    pattern.starts_with('/') && pattern.ends_with('/') && pattern.len() > 2
}

/// Parse a production line like "Lambda(lambda) ::= 'λ' Variable[x] ':' Type[τ₁] '.' Term[e]"
pub fn parse_production(line: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = line.splitn(2, "::=").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid production line: {}", line));
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

/// Parse nonterminal with optional rule name like "Lambda(lambda)" -> ("Lambda", Some("lambda"))
pub fn parse_nonterminal(nt_str: &str) -> Result<(String, Option<String>), String> {
    if let Some(open_paren) = nt_str.find('(') {
        if let Some(close_paren) = nt_str.rfind(')') {
            if close_paren > open_paren {
                let name = nt_str[..open_paren].trim().to_string();
                let rule_name = nt_str[open_paren + 1..close_paren].trim().to_string();
                return Ok((name, if rule_name.is_empty() { None } else { Some(rule_name) }));
            }
        }
    }
    // No rule name
    Ok((nt_str.trim().to_string(), None))
}

/// Parse RHS with bindings like "'λ' Variable[x] ':' Type[τ₁] '.' Term[e]"
pub fn parse_rhs(rhs: &str) -> Result<Vec<Vec<Symbol>>, String> {
    let mut alternatives = Vec::new();
    
    // Split by | for alternatives
    for alt in rhs.split('|').map(str::trim).filter(|alt| !alt.is_empty()) {
        let mut symbols_in_alt = Vec::new();
        for token in alt.split_whitespace() {
            // Check if it's a regex pattern first (before checking for bindings)
            if is_regex(token) {
                // It's a regex pattern like /[a-zA-Z]+/ - treat as regular symbol
                symbols_in_alt.push(Symbol::new(token.to_string()));
            } else if let Some(open_bracket) = token.find('[') {
                if let Some(close_bracket) = token.rfind(']') {
                    if close_bracket > open_bracket {
                        // Symbol with binding like "Variable[x]"
                        let value = token[..open_bracket].to_string();
                        let binding = token[open_bracket + 1..close_bracket].to_string();
                        symbols_in_alt.push(Symbol::with_binding(value, binding));
                        continue;
                    }
                }
                // If we get here, it has brackets but not a valid binding - treat as regular symbol
                symbols_in_alt.push(Symbol::new(token.to_string()));
            } else {
                // Regular symbol without binding
                symbols_in_alt.push(Symbol::new(token.to_string()));
            }
        }
        alternatives.push(symbols_in_alt);
    }
    
    Ok(alternatives)
}

/// Find special tokens in a right-hand side string.
pub fn special_tokens(rhs: &str) -> Vec<String> {
    let mut found = Vec::new();
    for alt in rhs.split('|').map(str::trim).filter(|alt| !alt.is_empty()) {
        for sym in alt.split_whitespace() {
            // Any token surrounded by single quotes is a special token
            if (sym.starts_with('\'') && sym.ends_with('\'')) || 
                (sym.starts_with('"') && sym.ends_with('"')) {
                let sym_stripped = sym.trim_matches('\'').trim_matches('"');
                if !found.contains(&sym_stripped.to_string()) {
                    found.push(sym_stripped.to_string());
                }
            }
            // Skip regex patterns - they're not special tokens
            else if is_regex(sym) {
                continue;
            }
        }
    }
    found
}

/// Parse a multi-line inference rule block
pub fn parse_inference_rule(lines: &[&str]) -> Result<TypingRule, String> {
    if lines.is_empty() {
        return Err("Empty rule block".into());
    }

    let mut premises = String::new();
    let mut conclusion = String::new();
    let mut name = String::new();
    let mut in_conclusion = false;

    // Regex that captures `(name)` only when the parentheses occur at end of string (optional trailing whitespace)
    let name_at_end = Regex::new(r"\(([^)]+)\)\s*$").unwrap();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.contains("---") {
            // dashed separator – start collecting conclusion next
            if let Some(cap) = name_at_end.captures(trimmed) {
                name = cap[1].trim().to_string();
            }
            in_conclusion = true;
            continue;
        }
        if !in_conclusion {
            premises = trimmed.to_string();
        } else {
            // first non-dash line after separator is conclusion
            conclusion = trimmed.to_string();
            // Try to extract rule name if not found yet and present at end of conclusion line
            if name.is_empty() {
                if let Some(cap) = name_at_end.captures(trimmed) {
                    name = cap[1].trim().to_string();
                    conclusion = name_at_end.replace(trimmed, "").trim().to_string();
                }
            }
        }
    }

    if name.is_empty() {
        return Err("Typing rule has no name".into());
    }

    Ok(TypingRule { name, premises, conclusion })
}