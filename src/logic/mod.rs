pub mod tokenizer;
pub mod ast;
pub mod grammar;
pub mod parser;
pub mod check;
pub mod utils;
pub mod typing;
pub mod serialize;
pub mod bind;
pub mod debug;



#[cfg(test)]
mod tests {

    use crate::logic::ast::ASTNode;

    use super::grammar::tests::STLC_SPEC;
    use super::grammar::Grammar;
    use super::parser::Parser;

    fn _load_stlc_grammar() -> Grammar {
        return Grammar::load(STLC_SPEC).expect("Parser failed");
    }

    #[test]
    pub fn test_parse_stlc() {
        let grammar = _load_stlc_grammar();
        
        // Test cases: (expression, expected_ast_sexpr) - Using actual canonical forms
        let test_cases = vec![
            // Simple application
            ("x y", r#"(N Term(N Application(rule app)(b e)(N BaseTerm(b f)(N Variable(N Identifier(b x)(T "x"))))(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "y"))))))"#),
            
            // Lambda abstraction  
            ("λx:A.x", r#"(N Term(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "x")))(T ":")(N Type(b τ)(N BaseType(b τ)(N TypeName(N Identifier(T "A"))))))(T ".")(N Term(b e)(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x"))))))))"#),
            
            // Simple variable
            ("x", r#"(N Term(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x")))))"#),
        ];

        for (input, expected_sexpr) in test_cases {
            let mut parser = Parser::new(grammar.clone());
            let parsed = parser
                .parse(input)
                .expect(&format!("Parse failed for '{}'", input));

            // Parse expected AST from S-expression
            let expected_ast = ASTNode::parse(expected_sexpr, &grammar)
                .expect(&format!("Failed to parse expected S-expression for '{}'", input));

            // Compare AST structures
            assert!(parsed.syneq(&expected_ast), "Parsed AST does not match expected AST for '{}': \n {} vs {}", input, parsed.serialize(), expected_sexpr);

            // Also test serialization roundtrip
            let sexpr = parsed.serialize();
            let new_ast = ASTNode::parse(&sexpr, &grammar)
                .expect(&format!("Failed to parse S-expression for '{}'", input));

            assert!(new_ast.syneq(&parsed), "Serialization roundtrip failed for '{}'", input);
        }
    }

    #[test]
    pub fn test_parse_stlc_complex() {
        let grammar = _load_stlc_grammar();
        
        // More complex test cases with expected AST structures
        let complex_test_cases = vec![
            // Nested lambda expressions
            ("λx:A.λy:B.x", r#"(N Term(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "x")))(T ":")(N Type(b τ)(N BaseType(b τ)(N TypeName(N Identifier(T "A"))))))(T ".")(N Term(b e)(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "y")))(T ":")(N Type(b τ)(N BaseType(b τ)(N TypeName(N Identifier(T "B"))))))(T ".")(N Term(b e)(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x"))))))))))))"#),
            
            ("λf:A->B.λx:A.f x", r#"(N Term(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "f")))(T ":")(N Type(b τ)(N BaseType(b τ₁)(N TypeName(N Identifier(T "A"))))(T "->")(N Type(b τ₂)(N BaseType(b τ)(N TypeName(N Identifier(T "B")))))))(T ".")(N Term(b e)(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "x")))(T ":")(N Type(b τ)(N BaseType(b τ)(N TypeName(N Identifier(T "A"))))))(T ".")(N Term(b e)(N Application(rule app)(b e)(N BaseTerm(b f)(N Variable(N Identifier(b x)(T "f"))))(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x"))))))))))))"#),
            
            // Binary applications
            ("f g", r#"(N Term(N Application(rule app)(b e)(N BaseTerm(b f)(N Variable(N Identifier(b x)(T "f"))))(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "g"))))))"#),
            
            // Single variable (deeply nested parentheses case)
            ("((((x))))", r#"(N Term(N BaseTerm(b e)(T "(")(N Term(N BaseTerm(b e)(T "(")(N Term(N BaseTerm(b e)(T "(")(N Term(N BaseTerm(b e)(T "(")(N Term(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x")))))(T ")")))(T ")")))(T ")")))(T ")"))))"#),
            
            // Unicode type variables
            ("λx:τ₁.x", r#"(N Term(N BaseTerm(b e)(N Lambda(rule lambda)(T "λ")(N TypedParam(N Variable(b x)(N Identifier(b x)(T "x")))(T ":")(N Type(b τ)(N BaseType(b τ)(N TypeName(N Identifier(T "τ₁"))))))(T ".")(N Term(b e)(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x"))))))))"#),
        ];

        for (input, expected_sexpr) in complex_test_cases {
            let mut parser = Parser::new(grammar.clone());
            let parsed = parser
                .parse(input)
                .expect(&format!("Parse failed for complex input '{}'", input));

            // Parse expected AST from S-expression
            let expected_ast = ASTNode::parse(expected_sexpr, &grammar)
                .expect(&format!("Failed to parse expected S-expression for complex input '{}'", input));

            assert!(parsed.syneq(&expected_ast), "Parsed AST does not match expected AST for complex input '{}'", input);

            // Serialization roundtrip test
            let sexpr = parsed.serialize();
            let new_ast = ASTNode::parse(&sexpr, &grammar)
                .expect(&format!("Failed to parse S-expression for complex input '{}' : \n {} \n {}", input, sexpr,expected_sexpr));

            assert!(new_ast.syneq(&parsed), "Serialization roundtrip failed for complex input '{}'", input);
        }
    }

    #[test] 
    pub fn test_parse_stlc_edge_cases() {
        let grammar = _load_stlc_grammar();
        
        // Edge cases with expected AST structures - Using actual canonical forms
        let edge_test_cases = vec![
            // Single identifiers
            ("x", r#"(N Term(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "x")))))"#),
            ("f", r#"(N Term(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "f")))))"#),
            
            // Binary applications
            ("a b", r#"(N Term(N Application(rule app)(b e)(N BaseTerm(b f)(N Variable(N Identifier(b x)(T "a"))))(N BaseTerm(b e)(N Variable(N Identifier(b x)(T "b"))))))"#),
        ];

        for (input, expected_sexpr) in edge_test_cases {
            let mut parser = Parser::new(grammar.clone());
            let parsed = parser
                .parse(input)
                .expect(&format!("Parse failed for edge case '{}'", input));

            // Parse expected AST from S-expression
            let expected_ast = ASTNode::parse(expected_sexpr, &grammar)
                .expect(&format!("Failed to parse expected S-expression for edge case '{}'", input));

            assert!(parsed.syneq(&expected_ast), "Parsed AST does not match expected AST for edge case '{}'", input);

            // Serialization roundtrip test
            let sexpr = parsed.serialize();
            let new_ast = ASTNode::parse(&sexpr, &grammar)
                .expect(&format!("Failed to parse S-expression for edge case '{}'", input));

            assert!(new_ast.syneq(&parsed), "Serialization roundtrip failed for edge case '{}'", input);
        }
    }

    #[test]
    pub fn test_parse_stlc_debug() {
        let grammar = _load_stlc_grammar();
        
        // Debug test to see actual AST structure
        let debug_inputs = vec!["λf:A->B.λx:A.f x"];
        
        for input in debug_inputs {
            let mut parser = Parser::new(grammar.clone());
            let parsed = parser
                .parse(input)
                .expect(&format!("Parse failed for '{}'", input));
            
            let _serialized = parsed.serialize();
            // Removed debug print - use unified debug system if needed
        }
    }


}



