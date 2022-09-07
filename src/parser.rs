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



