/*
 * Author: Dylan Turner
 * Description: Take string/repl input and turn it into an AST
 */

/*
 * EBNF:
 *
 * <stmt>           ::= <expr> | <func-def> | <asgn>
 * <func-def>       ::= '\' <ident> '(' [ <ident> { ',' <ident> } ] ')' '=' <expr>
 * <asgn>           ::= 'let' <ident> ':=' <expr>
 * <expr>           ::= <expr> | '(' <expr> ')'
 * <exp>            ::= <product> | <product> '^' <product>
 * <product>        ::= <sum> | <sum> ( '*' | '/' ) <sum>
 * <sum>            ::= <term> | <term> ( '+' | '-' ) <term>
 * <term>           ::= <ident> | <func-call> | <float> | <int>
 *                    | 'j' <term> | '-' <term> | <list>
 * <list>           ::= '[' [ <term> { ',' <term> } ] ']'
 * <func-call>      ::= <ident> '(' [ <expr> { ',' <expr> } ] ')'
 * <ident>          ::= /[A-Za-z_]+[A-Za-z_0-9]* /
 * <float>          ::= /\-?([0-9]+\.)?[0-9]+([Ee]\-?[0-9]+)?/
 * <int>            ::= /[0-9]+\_/
 */

#[derive(Debug, Clone)]
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
    List(Vec<Box<Token>>)
}

pub struct ParseResult {
    pub new_start: usize,
    pub token: Token
}

/* A series of helper functions for the parser */

pub fn retrieve_integer(code: &str) -> Option<ParseResult> {
    let mut int_str = String::new();
    let mut i = 0;
    while i < code.len() && (
        char::is_digit(code.chars().nth(i).unwrap(), 10) || code.chars().nth(i).unwrap() == '_'
    ) {
        if i == 0 && !char::is_digit(code.chars().nth(i).unwrap(), 10) {
            return None;
        }

        int_str.push(code.chars().nth(i).unwrap());
        i += 1;
    }

    if int_str.len() < 2 || !int_str.ends_with('_') {
        None
    } else {
        Some(ParseResult {
            new_start: i,
            token: Token::Integer(int_str.clone())
        })
    }
}

