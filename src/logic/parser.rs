
use crate::logic::grammar::{Grammar, Nonterminal, Production, Symbol};
use crate::logic::grammar::typing::TypingRule;
use crate::logic::ast::{ASTNode, NodeKind, SourceSpan};
use crate::logic::tokenizer::Tokenizer;
use regex;

/// A recursive-descent parser that uses a grammar to build an AST.
pub struct Parser {
    grammar: Grammar,
    pub tokenizer: Tokenizer,
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    pub fn new(grammar: Grammar) -> Self {
        // Create tokenizer with special tokens from grammar and common delimiters
        let tokenizer = Tokenizer::new(
            grammar.special_tokens.clone(),
            vec![' ', '\t', '\n', '\r'] // Common whitespace delimiters
        );
        
        Parser {
            grammar,
            tokenizer,
            tokens: vec![],
            pos: 0,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<ASTNode, String> {
        // Use proper tokenizer instead of simple whitespace splitting
        let token_ids = self.tokenizer.tokenize(input.to_string())
            .map_err(|_| "Tokenization failed".to_string())?;
        
        // Convert token IDs back to strings
        self.tokens = token_ids.iter()
            .filter_map(|&id| self.tokenizer.str(id))
            .collect();
        
        self.pos = 0;
        
        // Handle empty input
        if self.tokens.is_empty() {
            return Err("Empty input".to_string());
        }
        
        // Try to find a good starting nonterminal
        let start_nt = if self.grammar.productions.contains_key("Expr") {
            "Expr".to_string()
        } else if self.grammar.productions.contains_key("Term") {
            "Term".to_string()
        } else {
            self.grammar.productions.keys().next().cloned().unwrap_or_else(|| "Term".to_string())
        };
        
        // Try all productions for the start nonterminal
        if let Some(productions) = self.grammar.productions.get(&start_nt).cloned() {
            for production in productions {
                self.pos = 0; // Reset position for each attempt
                match self.try_production(&production) {
                    Ok(children) => {
                        // Check if all tokens were consumed
                        if self.pos >= self.tokens.len() {
                            let span = SourceSpan { start: 0, end: self.pos };
                            
                            // Get typing rule from grammar if a rule name is present
                            let typing_rule = if let Some(rule_name) = &production.rule {
                                self.grammar.typing_rules.get(rule_name).cloned()
                            } else {
                                None
                            };

                            let node = ASTNode {
                                kind: NodeKind::Nonterminal,
                                value: start_nt.clone(),
                                span: Some(span),
                                children: Some(children),
                                binding: None,
                                typing_rule,
                            };
                            return Ok(node);
                        }
                        // If not all tokens consumed, try next production
                    }
                    Err(_) => {
                        // Try next production
                        continue;
                    }
                }
            }
        }
        
        Err(format!("Unable to parse input completely"))
    }

    fn parse_nonterminal(&mut self, nt: &Nonterminal) -> Result<ASTNode, String> {
        if let Some(productions) = self.grammar.productions.get(nt).cloned() {
            for production in productions {
                let initial_pos = self.pos;
                match self.try_production(&production) {
                    Ok(children) => {
                        let span = SourceSpan { start: initial_pos, end: self.pos };

                        // Get typing rule from grammar if a rule name is present
                        let typing_rule = if let Some(rule_name) = &production.rule {
                            self.grammar.typing_rules.get(rule_name).cloned()
                        } else {
                            None
                        };

                        let node = ASTNode {
                            kind: NodeKind::Nonterminal,
                            value: nt.clone(),
                            span: Some(span),
                            children: Some(children),
                            binding: None, // Bindings are now on RHS symbols
                            typing_rule,
                        };
                        return Ok(node);
                    }
                    Err(_) => {
                        // Backtrack and try next production
                        self.pos = initial_pos;
                        continue;
                    }
                }
            }
        }
        Err(format!("Unable to parse nonterminal: {}", nt))
    }

    fn try_production(&mut self, production: &Production) -> Result<Vec<ASTNode>, String> {
        let mut children = Vec::new();
        for symbol in &production.rhs {
            match self.parse_symbol(symbol) {
                Ok(child) => children.push(child),
                Err(e) => return Err(e),
            }
        }
        Ok(children)
    }

    fn parse_symbol(&mut self, symbol: &Symbol) -> Result<ASTNode, String> {
        if self.pos >= self.tokens.len() {
            return Err("Unexpected end of input".to_string());
        }

        let token = &self.tokens[self.pos];
        
        // Check if this symbol is a nonterminal (exists in productions)
        let is_nonterminal = self.grammar.productions.contains_key(&symbol.value);
        
        if is_nonterminal {
            // It's a nonterminal, parse recursively
            self.parse_nonterminal(&symbol.value)
        } else {
            // It's a terminal, check if it matches
            let matches = if symbol.value.starts_with('\'') && symbol.value.ends_with('\'') {
                // Quoted terminal like 'λ' or '+'
                let expected = symbol.value.trim_matches('\'');
                expected == token
            } else if symbol.value.starts_with('/') && symbol.value.ends_with('/') {
                // Regex pattern like /[0-9]+/
                let pattern = symbol.value.trim_matches('/');
                match regex::Regex::new(pattern) {
                    Ok(re) => re.is_match(token),
                    Err(_) => false,
                }
            } else {
                // Direct match
                &symbol.value == token
            };
            
            if matches {
                let node = ASTNode {
                    kind: NodeKind::Terminal,
                    span: Some(SourceSpan { start: self.pos, end: self.pos + 1 }),
                    value: token.clone(),
                    children: None,
                    binding: symbol.binding.clone(),
                    typing_rule: None,
                };
                self.pos += 1;
                Ok(node)
            } else {
                Err(format!("Expected '{}', found '{}'", symbol.value, token))
            }
        }
    }
}