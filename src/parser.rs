/*
 * Author: Dylan Turner
 * Description:
 * - Take string/repl input and turn it into an AST
 * - In order to let tests.rs access some functions, there are a few helpers that are public, but
 *   in use, just parse_stmt is used
 */

/*
 * EBNF:
 *
 * <stmt>           ::= <expr> | <func-def> | <asgn>
 * <func-def>       ::= '\' <ident> '(' [ <ident> { ',' <ident> } ] ')' '=' <expr>
 * <asgn>           ::= 'let' <ident> ':=' <expr>
 * <expr>           ::= <exp> | '(' <expr> ')'
 * <exp>            ::= <product> | <product> '^' <product>
 * <product>        ::= <sum> | <sum> ( '*' | '/' ) <sum>
 * <sum>            ::= <term> | <term> ( '+' | '-' ) <term>
 * <term>           ::= <ident> | <func-call> | <float> | <int>
 *                    | 'j' <term> | '-' <term> | <list>
 * <list>           ::= '[' [ <term> { ',' <term> } ] ']'
 * <func-call>      ::= <ident> '(' [ <expr> { ',' <expr> } ] ')'
 * <ident>          ::= /[A-Za-z_]+[A-Za-z_0-9]* /
 * <float>          ::= /\-?([0-9]*\.)?[0-9]+([Ee]\-?[0-9]+)?/
 * <int>            ::= /-[0-9]+_/
 */

pub enum Token {
    Statement(Box<Token>),
    FunctionDefinition(String, Vec<String>, Box<Token>),
    Assignment(String, Box<Token>),
    Expr(Box<Token>),
    Exp(Box<Token>, Option<Box<Token>>),
    Product(Box<Token>, Option<char>, Option<Box<Token>>),
    Factor(Box<Token>, Option<char>, Option<Box<Token>>),
    Term(Box<Token>, Option<char>),
    Identifier(String),
    Number(String),
    Integer(String),
    List(Vec<Box<Token>>),
    Word(String)
}

pub struct ParseResult {
    pub new_start: usize,
    pub token: Token
}

/* A series of helper functions for the parser */

// <stmt> ::= <asgn> | <func-def> | <expr>
fn parse_stmt(code: &str) -> Option<ParseResult> {
    let attempt = parse_func_def(code);
    if attempt.is_some() {
        return attempt;
    }
    
    let attempt = parse_asgn(code);
    if attempt.is_some() {
        return attempt;
    }

    parse_expr(code)
}

// <func-def> ::= '\' <ident> '(' [ <ident> { ',' <ident> } ] ')' '=' <expr>
fn parse_func_def(code: &str) -> Option<ParseResult> {
    None
}

// <asgn> ::= 'let' <ident> ':=' <expr>
fn parse_asgn(code: &str) -> Option<ParseResult> {
    None
}

// <expr> ::= <exp> | '(' <expr> ')'
fn parse_expr(code: &str) -> Option<ParseResult> {
    None
}

// Get a specified string of characters
pub fn parse_word(word: &str, code: &str) -> Option<ParseResult> {
    if word.len() < code.len() {
        let mut i = 0;
        while i < word.len() {
            if word.chars().nth(i).unwrap() != code.chars().nth(i).unwrap() {
                return None;
            }
            i += 1;
        }
        Some(ParseResult {
            new_start: i,
            token: Token::Word(String::from(word))
        })
    } else {
        None        
    }
}

// <int> ::= /-?[0-9][0-9_]*_/
pub fn parse_integer(code: &str) -> Option<ParseResult> {
    let mut int_str = String::new();
    let mut i = 0;

    // Check for negative num
    if code.len() > 0 && code.chars().nth(0).unwrap() == '-' {
        int_str.push('-');
        i = 1;
    }

    // Get 0-9+, but allow inner '_' for breaking up big numbers
    while i < code.len() && (
        code.chars().nth(i).unwrap().is_digit(10) || code.chars().nth(i).unwrap() == '_'
    ) {
        // No starting with '_'
        if i == 0 && !code.chars().nth(i).unwrap().is_digit(10) {
            return None;
        }

        int_str.push(code.chars().nth(i).unwrap());
        i += 1;
    }

    // Finally, make sure it's a number and ends with _ and ship it
    if int_str.len() < 2 || !int_str.ends_with('_') {
        None
    } else {
        Some(ParseResult {
            new_start: i,
            token: Token::Integer(int_str.clone())
        })
    }
}

// <float> ::= /\-?([0-9]*\.)?[0-9]+([Ee]\-?[0-9]+)?/
pub fn parse_number(code: &str) -> Option<ParseResult> {
    let mut float_str = String::new();
    let mut i = 0;
    let mut found_pt = false;

    // Like int, check for negative
    if code.len() > 0 && code.chars().nth(0).unwrap() == '-' {
        float_str.push('-');
        i = 1;
    }

    // Get 0-9+ and 0-9+.0-9+
    while i < code.len() && (
        code.chars().nth(i).unwrap().is_digit(10)
            || (code.chars().nth(i).unwrap() == '.' && !found_pt)
    ) {
        if code.chars().nth(i).unwrap() == '.' {
            found_pt = true;
        }

        float_str.push(code.chars().nth(i).unwrap());
        i += 1;
    }

    // Scientific notation
    if i < code.len() && (
        code.chars().nth(i).unwrap() == 'E' || code.chars().nth(i).unwrap() == 'e'
        ) {
        // Might fail, so don't adjust i and str unless successful
        let mut j = i + 1;
        let mut e_str = String::from("E");
        
        // Handle negative in size
        if code.chars().nth(j).unwrap() == '-' {
            e_str.push('-');
            j += 1;
        }

        // Get the 0-9 part
        let mut num_str = String::new();
        while j < code.len() && code.chars().nth(j).unwrap().is_digit(10) {
            num_str.push(code.chars().nth(j).unwrap());
            j += 1;
        }

        if num_str.len() > 0 {
            // Success!
            i = j;
            float_str += e_str.as_str();
            float_str += num_str.as_str();
        }
    }

    if float_str.len() > 0 || float_str.chars().nth(0).unwrap() != '-' {
        Some(ParseResult {
            new_start: i,
            token: Token::Number(float_str.clone())
        })
    } else {
        None
    }
}

// <ident> ::= /[A-Za-z_]+[A-Za-z_0-9]*/
pub fn parse_ident(code: &str) -> Option<ParseResult> {
    let mut i = 0;
    let mut ident_str = String::new();

    // Make sure no num start
    if code.len() > 0 && (
        code.chars().nth(0).unwrap().is_alphabetic() || code.chars().nth(0).unwrap() == '_'
    ) {
        // Then get everything
        while i < code.len() && (
            code.chars().nth(i).unwrap().is_ascii_alphanumeric()
                || code.chars().nth(i).unwrap() == '_'
        ) {
            ident_str.push(code.chars().nth(i).unwrap());
            i += 1;
        }
    }

    if ident_str.len() > 0 {
        Some(ParseResult {
            new_start: i,
            token: Token::Identifier(ident_str.clone())
        })
    } else {
        None    
    }
}

