use crate::logic::check::parse;

pub const RELATION_SYMBOLS: [&str; 8] = ["=", "<", "∈", "⊆", "⊂", "⊃", "⊇", ":"];

pub fn parse_judgement(
    judgment_str: &str,
) -> Result<(Option<Vec<(String,String)>>, String,String), String> {

    // if we have a , after the context, it means we are doing extensions
    let parts: Vec<&str> = judgment_str.split('⊢').map(str::trim).collect();

    let extensions: Option<Vec<(String,String)>> = if parts[0].contains(',') {
        let (_base, exts) = parse_context_extensions(parts[0])?;
        Some(exts)
    } else {
        None
    };
    if parts.len() != 2 {
        return Err(format!("Invalid typing judgment format: {}", judgment_str));
    }

    // split the second part into expression and type
    let expr_parts: Vec<&str> = parts[1].split(':').map(str::trim).collect();
    if expr_parts.len() != 2 {
        return Err(format!("Invalid typing judgment format: {}", judgment_str));
    }

    Ok((extensions, expr_parts[0].to_string(), expr_parts[1].to_string()))
}

pub fn parse_membership(
    membership_str: &str,
) -> Result<(String, String), String> {
    let parts: Vec<&str> = membership_str.split('∈').map(str::trim).collect();
    if parts.len() != 2 {
        return Err(format!("Invalid membership format: {}", membership_str));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub fn parse_type_relation(
    relation_str: &str,
) -> Result<(String, String, String), String> {
    // we should have three parts
    // left <arbitrary symbols> right
    let mut left = String::new();
    let mut right = String::new();
    let mut relation = String::new();
    for c in relation_str.chars() {
        if RELATION_SYMBOLS.contains(&c.to_string().as_str()) {
            relation.push(c);
        } else {
            if relation.is_empty() {
                left.push(c);
            } else {
                right.push(c);
            }
        }
    }
    if relation.is_empty() {
        return Err(format!("No relation symbol found in: {}", relation_str));
    }
    if left.is_empty() || right.is_empty() {
        return Err(format!("Invalid type relation format: {}", relation_str));
    }
    Ok((left.trim().to_string(), right.trim().to_string(), relation))
}

/// Parses a context string like "Γ,x:τ₁,y:τ₂" into (base_context, Vec<(variable, type)>)
pub fn parse_context_extensions(context_str: &str) -> Result<(String, Vec<(String, String)>), String> {
    let context_str = context_str.trim();
    let mut parts = context_str.split(',');
    let base = parts.next().ok_or("Empty context string")?.trim();
    if base != "Γ" {
        return Err(format!("Context must start with 'Γ', got '{}'", base));
    }
    let mut extensions = Vec::new();
    for ext in parts {
        let ext = ext.trim();
        if ext.is_empty() { continue; }
        let ext_parts: Vec<&str> = ext.split(':').map(str::trim).collect();
        if ext_parts.len() != 2 {
            return Err(format!("Invalid context extension format, expected 'var:type': {}", ext));
        }
        let var = ext_parts[0].to_string();
        let ty = ext_parts[1].to_string();
        extensions.push((var, ty));
    }
    Ok((base.to_string(), extensions))
}