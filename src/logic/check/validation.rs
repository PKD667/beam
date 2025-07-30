
pub fn validate_type_expr(expr: &str) -> bool {
    // A very basic check for valid type expressions
    // This can be expanded with more complex rules as needed
    !expr.is_empty() && expr.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '→' || c == 'λ' || c == ':')
}

/// Validate conclusion format
pub fn validate_conclusion(conclusion: &str) -> Result<(), String> {
    let conclusion = conclusion.trim();
    // Can be a type expression or a typing judgment
    if conclusion.contains('⊢') {
        // Typing judgment format
        let parts: Vec<&str> = conclusion.split('⊢').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid conclusion typing judgment: {}", conclusion));
        }
    } else {
        // Type expression format
        if !validate_type_expr(conclusion) {
            return Err(format!("Invalid conclusion type expression: {}", conclusion));
        }
    }
    Ok(())
}