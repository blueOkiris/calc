/*
 * Author: Dylan Turner
 * Description: Parse expressions for calc
 */

/*
 * Pseudo-EBNF
 * 
 * <number> ::=     [0-9+] ('.' [0-9])
 * <term> ::=       <number> ('+' | '-') <number> | <number>
 * <factor> ::=     <term> ('*' | '/' | '%') <term> | <term>
 * <expr> ::=       <factor> | '(' <expr> ')' | '-' <expr> | '\' <expr> | /[a-z]/ ":=" <expr>
 * 
 * <keyword> ::=    '^' | "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "j"
 * <func> ::=       (<keyword> | "log_" /[0-9]+/) '(' <expr> ')'
 */
