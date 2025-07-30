# Unified Grammar Specification

A unified grammar format that combines syntax definitions with typing rules using standard mathematical inference notation with a **deterministic, unambiguous format**.

## Syntax

### Production Rules
```
NonTerminal(typing_rule) ::= syntax_rhs
```

### Typing Rules
```
premises
--------- (typing_rule)
conclusion
```

Where:
- **`NonTerminal(typing_rule)`**: Grammar rule name with typing rule reference in parentheses
- **`syntax_rhs`**: Traditional EBNF right-hand side with semantic bindings
- **`premises`**: Comma-separated conditions that must hold (above the line)
- **`-----`**: Horizontal inference bar
- **`(typing_rule)`**: Typing rule name (matches the (typing_rule) annotation)
- **`conclusion`**: The typing judgment this rule establishes (below the line)

### Comments
```
// This is a comment
```

### Semantic Bindings
Non-terminals in the RHS bind semantic variables that capture their meaning:
```
Lambda(lambda) ::= 'λ' Variable[x] ':' Type[τ] '.' Term[e]

//                      ^         ^^^^^^       ^^^^^^
//                      |         |            └─ binds semantic variable e
//                      |         └─ binds semantic variable τ  
//                      └─ binds semantic variable x

Γ,x:τ ⊢ e : σ
--------------------------- (lambda)
τ -> σ
```

## **DETERMINISTIC TYPING RULE FORMAT**

### **Strict Rule Structure**
Every typing rule MUST follow this exact format:
```
<premise1>, <premise2>, ..., <premiseN>
----------------------------------------- (<rule_name>)
<conclusion>
```

**Validation Rules:**
- Premises MUST be comma-separated (no other separators allowed)
- Rule name MUST match a production rule annotation exactly
- Conclusion MUST be a valid type expression or typing judgment
- All semantic variables MUST exist in the corresponding grammar production

### **Premise Types (Exhaustive List)**

#### 1. **Typing Judgment**: `<context> ⊢ <expr> : <type>`
**Format:** `Γ ⊢ e : τ` or `Γ,x:τ₁ ⊢ e : τ₂`

**Rules:**
- `<context>` MUST follow Context Format (see below)
- `<expr>` MUST be a semantic variable from grammar bindings
- `<type>` MUST be a valid type expression
- Exactly one `⊢` symbol required
- Exactly one `:` symbol required after `⊢`

**Valid Examples:**
```
Γ ⊢ e : τ
Γ,x:Int ⊢ e : Bool
Γ,x:τ₁,y:τ₂ ⊢ f : τ₁ -> τ₂
```

**Invalid Examples:**
```
Γ ⊢ e τ           // Missing :
Γ,x ⊢ e : τ       // Invalid context extension
⊢ e : τ           // Missing context
Γ ⊢ : τ           // Missing expression
```

#### 2. **Context Membership**: `<var> ∈ <context>`
**Format:** `x ∈ Γ`

**Rules:**
- `<var>` MUST be a valid semantic variable
- `<context>` MUST be `Γ` or a context variable
- Exactly one `∈` symbol required

**Valid Examples:**
```
x ∈ Γ
f ∈ Γ
```

**Invalid Examples:**
```
x y ∈ Γ          // Multiple variables
x ∈              // Missing context
∈ Γ              // Missing variable
```

#### 3. **Type Relation**: `<type1> = <type2>` or `<type1> ∈ <type2>`
**Format:** `τ₁ = τ₂`

**Rules:**
- Both sides MUST be valid type expressions
- Exactly one relation symbol required
- MUST NOT contain `⊢` (to avoid confusion with typing judgments)

**Valid Examples:**
```
τ₁ = τ₂
Int < σ
τ -> σ = α -> β
```

**Invalid Examples:**
```
τ₁ = τ₂ = τ₃     // Multiple equals
= τ              // Missing left side
τ =              // Missing right side
```

#### 4. **Custom Predicate**: `<predicate>(<args>)`
**Format:** `predicate(arg1, arg2, ...)`

**Rules:**
- `<predicate>` MUST be alphabetic characters only
- Arguments MUST be comma-separated semantic variables
- Parentheses are required (even for zero arguments)
- All arguments MUST be valid semantic variables

**Valid Examples:**
```
fresh(x)
unify(τ₁, τ₂)
occurs_check(α, τ)
constraint()
```

**Invalid Examples:**
```
fresh x          // Missing parentheses
unify(τ₁ τ₂)     // Missing comma
123_pred(x)      // Invalid predicate name
pred(not_var)    // Invalid argument format
```

### **Context Format (Strict Grammar)**
```
<context> ::= 'Γ'                                    // Base context
            | 'Γ,' <extension_list>                  // Extended context

<extension_list> ::= <extension>                      // Single extension
                   | <extension> ',' <extension_list> // Multiple extensions

<extension> ::= <var> ':' <type>                     // Variable binding

<var> ::= <semantic_variable>                        // Must be valid semantic variable
<type> ::= <type_expression>                         // Must be valid type expression
```

**Valid Context Examples:**
```
Γ                    // Base context
Γ,x:τ               // Single extension
Γ,x:τ₁,y:τ₂         // Multiple extensions
Γ,f:τ->σ,x:τ        // Function type in context
```

**Invalid Context Examples:**
```
x:τ                 // Missing Γ base
Γ x:τ               // Missing comma
Γ,x τ               // Missing colon
Γ,x:                // Missing type
Γ,:τ                // Missing variable
```

### **Type Expression Format (Strict Grammar)**
```
<type> ::= <type_var>                               // Type variable
         | <type_constructor>                       // Base type
         | <type> '->' <type>                       // Function type (right-associative)
         | '(' <type> ')'                           // Parenthesized type
         | <type_constructor> '<' <type_list> '>'   // Generic type

<type_var> ::= <semantic_variable>                  // τ, σ, α, etc.
<type_constructor> ::= [A-Z][a-zA-Z0-9]*           // Int, Bool, List, etc.
<type_list> ::= <type> | <type> ',' <type_list>    // Comma-separated types
```

**Valid Type Examples:**
```
τ                   // Type variable
Int                 // Base type
τ -> σ              // Function type
(τ -> σ) -> τ       // Parenthesized function type
List<Int>           // Generic type
```

**Invalid Type Examples:**
```
τ ->                // Incomplete function type
-> σ                // Missing left operand
τ → σ               // Wrong arrow symbol (use ->)
list<int>           // Lowercase type constructor
```

### **Semantic Variable Format (Strict Rules)**
Semantic variables MUST match exactly ONE of these patterns:

1. **Single Latin letter**: `a-z`, `A-Z`
   - Examples: `x`, `y`, `f`, `e`, `X`, `Y`

2. **Single Greek letter**: `α-ω`, `Α-Ω`
   - Examples: `τ`, `σ`, `α`, `β`, `Γ`, `Δ`

3. **Subscripted variable**: `<base><subscript>`
   - Base: Single Latin or Greek letter
   - Subscript: Unicode subscript digits `₀₁₂₃₄₅₆₇₈₉`
   - Examples: `τ₁`, `τ₂`, `e₁`, `x₀`, `α₁₂₃`

**Valid Semantic Variables:**
```
x, y, z, f, e, g, h
τ, σ, α, β, γ, δ, ρ
τ₁, τ₂, e₁, x₀, α₁₂₃
```

**Invalid Semantic Variables:**
```
var         // Multi-character
x1          // Regular digits (use τ₁)
τ_1         // Underscore not allowed
xy          // Multiple characters
τα          // Multiple base characters
```

### **Conclusion Format**
The conclusion MUST be one of:

1. **Type Expression**: Following Type Expression Format
   ```
   τ -> σ
   Int
   List<τ>
   ```

2. **Typing Judgment**: Following Typing Judgment Format
   ```
   Γ ⊢ e : τ
   Γ,x:σ ⊢ f : τ -> σ
   ```

## Grammar Elements

### Terminals
- **String literals**: `'λ'`, `'('`, `')'`, `':'`, `'->'`
- **Regular expressions**: `/[a-zA-Z][a-zA-Z0-9_]*/`

### Non-terminals
- **With semantic binding**: `Variable[x]`, `Type[τ]`, `Term[e]`
- **Without binding**: `Variable`, `Type`, `Term` (when binding not needed)

### Alternatives
Use `|` to separate alternatives in the same production:
```
Type ::= AtomicType[τ] | FunctionType[τ]
```

### Pure Syntax Rules
Productions without typing rules don't need parentheses:
```
TypeName ::= /[a-zA-Z][a-zA-Z0-9_]*/
```

## Example: Simply Typed Lambda Calculus (Deterministic Format)

### Production Rules

```
// Variables  
Variable(var) ::= /[a-zA-Z][a-zA-Z0-9_]*/[x]

// Type names
TypeName ::= /[A-Z][a-zA-Z0-9_]*/

// Atomic types
AtomicType ::= TypeName | '(' Type ')'

// Function types
FunctionType ::= AtomicType[τ] '->' Type[σ]

Type ::= AtomicType 
       | FunctionType

// Typed parameter
TypedParam ::= Variable[x] ':' Type[τ]

// Lambda abstraction
Lambda(lambda) ::= 'λ' TypedParam '.' Term[e]

// Function application
Application(app) ::= Term[f] Term[e]

// Terms
Term ::= Variable | Lambda | Application
```

### Typing Rules (Deterministic Format)

```
x ∈ Γ
------------ (var)
Γ(x)

Γ,x:τ ⊢ e : σ
--------------------------- (lambda)
τ -> σ

Γ ⊢ f : τ -> σ, Γ ⊢ e : τ
-------------------------------- (app)  
σ
```

## **Validation and Error Handling**

### **Rule Validation**
A typing rule is **INVALID** and MUST be rejected if:

1. **Format Violations:**
   - Premises not comma-separated
   - Missing or malformed inference bar
   - Rule name doesn't match production annotation
   - Invalid premise type

2. **Context Format Violations:**
   - Context doesn't start with `Γ`
   - Invalid extension format (not `var:type`)
   - Invalid variable or type in extension

3. **Semantic Variable Violations:**
   - Variables don't match semantic variable format
   - Variables in premises don't exist in grammar production
   - Invalid subscript usage

4. **Type Expression Violations:**
   - Invalid type constructor format
   - Malformed function types
   - Invalid generic type syntax

5. **Premise-Specific Violations:**
   - Typing judgment: Missing `⊢` or `:`, invalid format
   - Membership: Missing `∈`, invalid variable/context
   - Type equality: Multiple `=` symbols, contains `⊢`
   - Custom predicate: Missing parentheses, invalid arguments

### **Error Messages**
The type checker MUST provide specific error messages for each violation type:

```
"Invalid typing judgment format: missing ⊢ symbol"
"Invalid context extension format, expected 'var:type': x τ"
"Invalid semantic variable format: var123 (use subscripts like x₁₂₃)"
"Missing required binding 'e' for rule 'lambda'"
"Premise failed for rule 'app': Γ ⊢ f : τ -> σ"
```

