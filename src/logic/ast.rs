use super::grammar::typing::TypingRule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Terminal,
    Nonterminal,
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub kind: NodeKind,
    pub value: String,
    pub span: Option<SourceSpan>,

    // only for non-terminals
    pub children: Option<Vec<ASTNode>>,

    // type shit
    pub binding: Option<String>,
    pub typing_rule: Option<TypingRule>,
}

impl ASTNode {
    pub fn span(&self) -> Option<&SourceSpan> {
        self.span.as_ref()
    }

    pub fn set_span(&mut self, new_span: SourceSpan) {
        self.span = Some(new_span);
    }

    pub fn binding(&self) -> Option<&String> {
        self.binding.as_ref()
    }


    pub fn pretty_print(&self, indent: usize) -> String {
        let indent_str = "  ".repeat(indent);
        match self.kind {
            NodeKind::Terminal => {
                let mut result = if let Some(b) = &self.binding {
                    format!("{}'{}'[{}]", indent_str, self.value, b)
                } else {
                    format!("{}'{}'", indent_str, self.value)
                };
                if let Some(rule) = &self.typing_rule {
                    result.push_str(&format!("@{}", rule.name));
                }
                result
            }
            NodeKind::Nonterminal => {
                let mut result = if let Some(b) = &self.binding {
                    format!("{}{}[{}]", indent_str, self.value, b)
                } else {
                    format!("{}{}", indent_str, self.value)
                };
                if let Some(rule) = &self.typing_rule {
                    result.push_str(&format!("@{}", rule.name));
                }
                if let Some(children) = &self.children {
                    if !children.is_empty() {
                        result.push_str(" (\n");
                        for child in children {
                            result.push_str(&child.pretty_print(indent + 1));
                            result.push('\n');
                        }
                        result.push_str(&format!("{})", indent_str));
                    }
                }
                result
            }
        }
    }
}