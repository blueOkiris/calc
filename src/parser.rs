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
 * <func-def>       ::= '\' <ident> '(' [ <ident> { ',' <ident> } ] ')' '->' <expr>
 * <asgn>           ::= 'let' <ident> ':=' <expr>
 * <expr>           ::= <un-expr> [ '?' <expr> ':' <expr> ]
 * <un-expr>        ::= <exp-expr> | 'j' <exp-expr> | '-' <exp-expr>
 * <exp-expr>       ::= <prod-expr> [ '^' <prod-expr> ]
 * <prod-expr>      ::= <sum-expr> [ ( '*' | '/' ) <sum-expr> ]
 * <sum-expr>       ::= <rel-expr> [ ( '+' | '-' ) <rel-expr> ]
 * <rel-expr>       ::= <term> [ ('=' | '=/=' | '>' | '<' | '>=' | '<=' ) <term> ]
 * <term>           ::= <ident> | <float> | <int> | <list> | <func-call> | '(' <expr> ')'
 * <list>           ::= '[' [ <expr> { ',' <expr> } ] ']'
 * <func-call>      ::= <ident> '(' [ <expr> { ',' <expr> } ] ')'
 * <ident>          ::= /[A-Za-z_]+[A-Za-z_0-9]* /
 * <float>          ::= /([0-9]*\.)?[0-9]+([Ee]\-?[0-9]+)?/
 * <int>            ::= /[0-9]+_/
 */

#[derive(Clone, Debug)]
pub enum Token {
    Statement(Box<Token>),
    FunctionDefinition(String, Vec<String>, Box<Token>),
    Assignment(String, Box<Token>),
    Expression(Box<Token>, Option<Box<Token>>, Option<Box<Token>>),
    UnaryExpression(Box<Token>, Option<String>),
    ExponentialExpression(Box<Token>, Option<Box<Token>>),
    ProductExpression(Box<Token>, Option<String>, Option<Box<Token>>),
    SumExpression(Box<Token>, Option<String>, Option<Box<Token>>),
    RelationalExpression(Box<Token>, Option<String>, Option<Box<Token>>),
    Term(Box<Token>),
    Identifier(String),
    Number(String),
    Integer(String),
    List(Vec<Box<Token>>),
    FunctionCall(String, Vec<Box<Token>>),
    Word(String),
    Whitespace
}

// Could use error, could use just Option<Token> and modify end val, but I think this is the best
#[derive(Clone)]
pub struct ParseResult {
    pub new_start: usize,
    pub token: Token
}

// Primary parsing function:
// <stmt> ::= <asgn> | <func-def> | <expr>
pub fn parse_stmt(code: &str) -> Result<Token, String> {
    let attempt = parse_func_def(code);
    if attempt.is_some() {
        if attempt.clone().unwrap().new_start < code.len() {
            return Err(format!(
                "Extra characters at end of expression starting at {}",
                attempt.unwrap().new_start
            ))
        } else {
            return Ok(Token::Statement(Box::new(attempt.unwrap().token)));
        }
    }
    
    let attempt = parse_asgn(code);
    if attempt.is_some() {
        if attempt.clone().unwrap().new_start < code.len() {
            return Err(format!(
                "Extra characters at end of expression starting at {}",
                attempt.unwrap().new_start
            ))
        } else {
            return Ok(Token::Statement(Box::new(attempt.unwrap().token)));
        }
    }

    match parse_expr(code, 0) {
        Err(err) => Err(err),
        Ok(expr) => if expr.new_start < code.len() {
            Err(format!("Extra characters at end of expression starting at {}", expr.new_start))
        } else {
            Ok(Token::Statement(Box::new(expr.token)))
        }
    }
}

/* Helpers for each statement */

// <func-def> ::= '\' <ident> '(' [ <ident> { ',' <ident> } ] ')' '=' <expr>
fn parse_func_def(code: &str) -> Option<ParseResult> {
    let mut substr_start;

    // '\'
    let lambda = parse_word("\\", code);
    if lambda.is_none() {
        return None;
    }
    substr_start = lambda.unwrap().new_start;

    let name = parse_ident(code.split_at(substr_start).1);
    if name.is_none() {
        return None;
    }
    substr_start += name.clone().unwrap().new_start;
    let name_str = if let Token::Identifier(try_name_str) = name.unwrap().token {
        try_name_str
    } else {
        String::new()
    };

    // '('
    let par = parse_word("(", code.split_at(substr_start).1);
    if par.is_none() {
        return None;
    }
    substr_start += par.unwrap().new_start;

    // <ident> { ',' <ident> } ]
    let mut args = Vec::new();
    let try_arg = parse_ident(code.split_at(substr_start).1);
    if try_arg.is_some() {
        substr_start += try_arg.clone().unwrap().new_start;
        args.push(
            if let Token::Identifier(try_arg_str) = try_arg.unwrap().token {
                try_arg_str
            } else {
                String::new()
            }
        );

        loop {
            let comma = parse_word(",", code.split_at(substr_start).1);
            if comma.is_none() {
                break;
            }
            substr_start += comma.unwrap().new_start;

            let arg = parse_ident(code.split_at(substr_start).1);
            if arg.is_none() {
                return None;
            }
            substr_start += arg.clone().unwrap().new_start;
            args.push(
                if let Token::Identifier(arg_str) = arg.unwrap().token {
                    arg_str
                } else {
                    String::new()
                }
            );
        }
    }

    // ')'
    let par = parse_word(")", code.split_at(substr_start).1);
    if par.is_none() {
        return None;
    }
    substr_start += par.unwrap().new_start;

    // '->'
    let eq = parse_word("->", code.split_at(substr_start).1);
    if eq.is_none() {
        return None;
    }
    substr_start += eq.unwrap().new_start;

    let expr = parse_expr(code.split_at(substr_start).1, substr_start);
    if expr.is_err() {
        return None;
    }
    substr_start += expr.clone().unwrap().new_start;

    Some(ParseResult {
        new_start: substr_start,
        token: Token::FunctionDefinition(
            name_str.clone(), args, Box::new(expr.unwrap().token)
        )
    })
}

// <asgn> ::= 'let' <ident> ':=' <expr>
fn parse_asgn(code: &str) -> Option<ParseResult> {
    let mut substr_start;

    // 'let'
    let keyword = parse_word("let", code);
    if keyword.is_none() {
        return None;
    }
    substr_start = keyword.unwrap().new_start;

    let name = parse_ident(code.split_at(substr_start).1);
    if name.is_none() {
        return None;
    }
    substr_start += name.clone().unwrap().new_start;
    let name_str = if let Token::Identifier(try_name_str) = name.unwrap().token {
        try_name_str
    } else {
        String::new()
    };

    // '='
    let eq = parse_word("=", code.split_at(substr_start).1);
    if eq.is_none() {
        return None;
    }
    substr_start += eq.unwrap().new_start;

    let expr = parse_expr(code.split_at(substr_start).1, substr_start);
    if expr.is_err() {
        return None;
    }
    substr_start += expr.clone().unwrap().new_start;

    Some(ParseResult {
        new_start: substr_start,
        token: Token::Assignment(
            name_str.clone(), Box::new(expr.unwrap().token)
        )
    })
}

/* Expressionession Parser */

// <expr> ::= <un-expr> [ '?' <expr> ':' <expr> ]
fn parse_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let unary = parse_un_expr(code, pos);
    match unary {
        Err(err) => Err(err),
        Ok(unary_res) => {
            let q = parse_word("?", code.split_at(unary_res.new_start).1);
            if q.is_none() {
                Ok(ParseResult {
                    new_start: unary_res.new_start,
                    token: Token::Expression(Box::new(unary_res.token), None, None)
                })
            } else {
                let mut substr_start = unary_res.new_start + q.unwrap().new_start;

                let t_expr = parse_expr(code.split_at(substr_start).1, pos + substr_start);
                if t_expr.is_err() {
                    return Err(format!(
                        "Expected expression after '?' at pos {}", pos + substr_start
                    ));
                }
                substr_start += t_expr.clone().unwrap().new_start;

                let coln = parse_word(":", code.split_at(substr_start).1);
                if coln.is_none() {
                    return Err(format!("Expected ':' at pos {}", pos + substr_start));
                }
                substr_start += coln.unwrap().new_start;

                let f_expr = parse_expr(code.split_at(substr_start).1, pos + substr_start);
                if f_expr.is_err() {
                    return Err(format!(
                        "Expected expression after ':' at pos {}", pos + substr_start
                    ));
                }
                substr_start += f_expr.clone().unwrap().new_start;

                Ok(ParseResult {
                    new_start: substr_start,
                    token: Token:: Expression(
                        Box::new(unary_res.token),
                        Some(Box::new(t_expr.unwrap().token)),
                        Some(Box::new(f_expr.unwrap().token))
                    )
                })
            }
        }
    }
}

// <un-expr> ::= <exp-expr> | 'j' <exp-expr> | '-' <exp-expr>
fn parse_un_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let mut substr_start = 0;

    let ops = [ "j", "-" ];
    let mut atmpt = None;
    let mut used_op = "";
    for op in ops {
        atmpt = parse_word(op, code);
        if atmpt.is_some() {
            used_op = op;
            break;
        }
    }
    if atmpt.is_some() {
        substr_start = atmpt.clone().unwrap().new_start;
    }

    let exp = parse_exp_expr(code.split_at(substr_start).1, pos + substr_start);
    if exp.is_err() {
        return Err(exp.err().unwrap());
    }
    substr_start += exp.clone().unwrap().new_start;

    return Ok(ParseResult {
        new_start: substr_start,
        token: Token::UnaryExpression(
            Box::new(exp.unwrap().token),
            if atmpt.is_some() {
                Some(String::from(used_op))
            } else {
                None
            }
        )
    })
}

// <exp-expr> ::= <prod-expr> [ '^' <prod-expr> ]
fn parse_exp_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let mut substr_start;

    let fst = parse_prod_expr(code, pos);
    if fst.is_err() {
        return Err(fst.err().unwrap());
    }
    substr_start = fst.clone().unwrap().new_start;

    let atmpt = parse_word("^", code.split_at(substr_start).1);
    if atmpt.is_none() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::ExponentialExpression(Box::new(fst.unwrap().token), None)
        });
    }
    substr_start += atmpt.unwrap().new_start;

    // We found the operator, let's get the next token
    let snd = parse_prod_expr(code.split_at(substr_start).1, pos + substr_start);
    if snd.is_err() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::ExponentialExpression(Box::new(fst.unwrap().token), None)
        });
    }
    substr_start += snd.clone().unwrap().new_start;

    return Ok(ParseResult {
        new_start: substr_start,
        token: Token::ExponentialExpression(
            Box::new(fst.unwrap().token),
            Some(Box::new(snd.unwrap().token))
        )
    })
}

// <prod-expr> ::= <sum-expr> [ ( '*' | '/' ) <sum-expr> ]
fn parse_prod_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let mut substr_start;

    let fst = parse_sum_expr(code, pos);
    if fst.is_err() {
        return Err(fst.err().unwrap());
    }
    substr_start = fst.clone().unwrap().new_start;

    let ops = [ "*", "/" ];
    let mut atmpt = None;
    let mut used_op = "";
    for op in ops {
        atmpt = parse_word(op, code.split_at(substr_start).1);
        if atmpt.is_some() {
            used_op = op;
            break;
        }
    }
    if atmpt.is_none() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::ProductExpression(Box::new(fst.unwrap().token), None, None)
        });
    }
    substr_start += atmpt.unwrap().new_start;

    // We found the operator, let's get the next token
    let snd = parse_sum_expr(code.split_at(substr_start).1, pos + substr_start);
    if snd.is_err() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::ProductExpression(Box::new(fst.unwrap().token), None, None)
        });
    }
    substr_start += snd.clone().unwrap().new_start;

    return Ok(ParseResult {
        new_start: substr_start,
        token: Token::ProductExpression(
            Box::new(fst.unwrap().token),
            Some(String::from(used_op)),
            Some(Box::new(snd.unwrap().token))
        )
    })
}

// <sum-expr> ::= <rel-expr> [ ( '+' | '-' ) <rel-expr> ]
fn parse_sum_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let mut substr_start;

    let fst = parse_rel_expr(code, pos);
    if fst.is_err() {
        return Err(fst.err().unwrap());
    }
    substr_start = fst.clone().unwrap().new_start;

    let ops = [ "+", "-" ];
    let mut atmpt = None;
    let mut used_op = "";
    for op in ops {
        atmpt = parse_word(op, code.split_at(substr_start).1);
        if atmpt.is_some() {
            used_op = op;
            break;
        }
    }
    if atmpt.is_none() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::SumExpression(Box::new(fst.unwrap().token), None, None)
        });
    }
    substr_start += atmpt.unwrap().new_start;

    // We found the operator, let's get the next token
    let snd = parse_rel_expr(code.split_at(substr_start).1, pos + substr_start);
    if snd.is_err() {
        return Ok(ParseResult {
            new_start: fst.clone().unwrap().new_start,
            token: Token::SumExpression(Box::new(fst.unwrap().token), None, None)
        });
    }
    substr_start += snd.clone().unwrap().new_start;

    return Ok(ParseResult {
        new_start: substr_start,
        token: Token::SumExpression(
            Box::new(fst.unwrap().token),
            Some(String::from(used_op)),
            Some(Box::new(snd.unwrap().token))
        )
    })
}

// <rel-expr> ::= <term> [ ('=' | '=/=' | '>' | '<' | '>=' | '<=' ) <term> ]
fn parse_rel_expr(code: &str, pos: usize) -> Result<ParseResult, String> {
    let mut substr_start;

    let first = parse_term(code, pos);
    if first.is_err() {
        return Err(first.err().unwrap());
    }
    substr_start = first.clone().unwrap().new_start;

    // Optional '==', '=/=', '>', ...
    let ops = [ "=/=", "=", ">=", "<=", ">", "<" ];
    let mut atmpt = None;
    let mut used_op = "";
    for op in ops {
        atmpt = parse_word(op, code.split_at(substr_start).1);
        if atmpt.is_some() {
            used_op = op;
            break;
        }
    }
    if atmpt.is_none() {
        return Ok(ParseResult {
            new_start: first.clone().unwrap().new_start,
            token: Token::RelationalExpression(Box::new(first.unwrap().token), None, None)
        });
    }
    substr_start += atmpt.unwrap().new_start;

    // We found the operator, let's get the next token
    let snd = parse_term(code.split_at(substr_start).1, pos + substr_start);
    if snd.is_err() {
        return Ok(ParseResult {
            new_start: first.clone().unwrap().new_start,
            token: Token::RelationalExpression(Box::new(first.unwrap().token), None, None)
        });
    }
    substr_start += snd.clone().unwrap().new_start;

    return Ok(ParseResult {
        new_start: substr_start,
        token: Token::RelationalExpression(
            Box::new(first.unwrap().token),
            Some(String::from(used_op)),
            Some(Box::new(snd.unwrap().token))
        )
    })
}

// <term> ::= <ident> | <float> | <int> | <list> | <func-call> | '(' <expr> ')'
fn parse_term(code: &str, pos: usize) -> Result<ParseResult, String> {
    // Check for parenth
    let par = parse_word("(", code);
    if par.is_some() {
        // '(' <expr> ')'
        let mut substr_start = par.unwrap().new_start;
        
        let sub_expr = parse_expr(code.split_at(substr_start).1, pos + substr_start);
        if sub_expr.is_err() {
            return Err(sub_expr.err().unwrap());
        }
        substr_start += sub_expr.clone().unwrap().new_start;

        let par = parse_word(")", code.split_at(substr_start).1);
        if par.is_none() {
            return Err(format!("Missing ')' at pos {}", substr_start + pos));
        } else {
           return Ok(ParseResult {
                new_start: substr_start + par.unwrap().new_start,
                token: Token::Term(Box::new(sub_expr.unwrap().token))
            });
        }
    }

    let atmpt = parse_list(code, pos);
    if atmpt.is_some() {
        return Ok(ParseResult {
            new_start: atmpt.clone().unwrap().new_start,
            token: Token::Term(Box::new(atmpt.unwrap().token))
        });
    }

    let atmpt = parse_func_call(code, pos);
    if atmpt.is_some() {
        return Ok(ParseResult {
            new_start: atmpt.clone().unwrap().new_start,
            token: Token::Term(Box::new(atmpt.unwrap().token))
        });
    }

    let atmpt = parse_ident(code);
    if atmpt.is_some() {
        return Ok(ParseResult {
            new_start: atmpt.clone().unwrap().new_start,
            token: Token::Term(Box::new(atmpt.unwrap().token))
        });
    }

    let atmpt = parse_integer(code);
    if atmpt.is_some() {
        return Ok(ParseResult {
            new_start: atmpt.clone().unwrap().new_start,
            token: Token::Term(Box::new(atmpt.unwrap().token))
        });
    }

    let atmpt = parse_number(code);
    if atmpt.is_some() {
        return Ok(ParseResult {
            new_start: atmpt.clone().unwrap().new_start,
            token: Token::Term(Box::new(atmpt.unwrap().token))
        });
    }

    Err(format!("Expected term at pos {}", pos))
}

/* Complex terms (i.e. uses base terms, but not quite into actual expr building yet) */

// <list> ::= '[' [ <expr> { ',' <expr> } ] ']'
fn parse_list(code: &str, pos: usize) -> Option<ParseResult> {
    let mut items = Vec::new();
    let mut substr_start;

    let brack = parse_word("[", code);
    if brack.is_none() {
        return None;
    }
    substr_start = brack.unwrap().new_start;

    let first_item = parse_expr(code.split_at(substr_start).1, pos + substr_start);
    if first_item.is_ok() {
        items.push(Box::new(first_item.clone().unwrap().token));
        substr_start += first_item.unwrap().new_start;

        loop {
            let comma = parse_word(",", code.split_at(substr_start).1);
            if comma.is_none() {
                break;
            }
            substr_start += comma.unwrap().new_start;

            let item = parse_expr(code.split_at(substr_start).1, pos + substr_start);
            if item.is_err() {
                return None;
            }
            items.push(Box::new(item.clone().unwrap().token));
            substr_start += item.unwrap().new_start;
        }
    }

    let brack = parse_word("]", code.split_at(substr_start).1);
    if brack.is_none() {
        return None;
    }
    substr_start += brack.unwrap().new_start;

    Some(ParseResult {
        new_start: substr_start,
        token: Token::List(items)
    })
}

// <func-call> ::= <ident> '(' [ <expr> { ',' <expr> } ] ')'
fn parse_func_call(code: &str, pos: usize) -> Option<ParseResult> {
    let mut substr_start;
    let mut args = Vec::new();

    let fname = parse_ident(code);
    if fname.is_none() {
        return None;
    }
    substr_start = fname.clone().unwrap().new_start;
    let fname_str = if let Token::Identifier(try_fname_str) = fname.unwrap().token {
        try_fname_str
    } else {
        String::new()
    };

    // '('
    let par = parse_word("(", code.split_at(substr_start).1);
    if par.is_none() {
        return None;
    }
    substr_start += par.unwrap().new_start;

    // Expression list
    let first_arg = parse_expr(code.split_at(substr_start).1, pos + substr_start);
    if first_arg.is_ok() {
        args.push(Box::new(first_arg.clone().unwrap().token));
        substr_start += first_arg.unwrap().new_start;

        loop {
            let comma = parse_word(",", code.split_at(substr_start).1);
            if comma.is_none() {
                break;
            }
            substr_start += comma.unwrap().new_start;

            let arg = parse_expr(code.split_at(substr_start).1, pos + substr_start);
            if arg.is_err() {
                return None;
            }
            args.push(Box::new(arg.clone().unwrap().token));
            substr_start += arg.unwrap().new_start;
        }
    }
    
    let par = parse_word(")", code.split_at(substr_start).1);
    if par.is_none() {
        return None;
    }
    substr_start += par.unwrap().new_start;

    Some(ParseResult {
        new_start: substr_start,
        token: Token::FunctionCall(fname_str, args)
    })
}

/* Fundamental, underlying data types */

// <int> ::= /-?[0-9][0-9_]*_/
fn parse_integer(code: &str) -> Option<ParseResult> {
    let mut int_str = String::new();
    let mut i = 0;

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
        let skip_ws = parse_whitespace(code.split_at(i).1);
        i += skip_ws.new_start;
        Some(ParseResult {
            new_start: i,
            token: Token::Integer(int_str.clone())
        })
    }
}

// <float> ::= /\-?([0-9]*\.)?[0-9]+([Ee]\-?[0-9]+)?/
fn parse_number(code: &str) -> Option<ParseResult> {
    let mut float_str = String::new();
    let mut i = 0;
    let mut found_pt = false;

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

    if float_str.len() > 0 {
        let skip_ws = parse_whitespace(code.split_at(i).1);
        i += skip_ws.new_start;
        Some(ParseResult {
            new_start: i,
            token: Token::Number(float_str.clone())
        })
    } else {
        None
    }
}

// <ident> ::= /[A-Za-z_]+[A-Za-z_0-9]*/
fn parse_ident(code: &str) -> Option<ParseResult> {
    let mut i = 0;
    let mut ident_str = String::new();

    // Make sure no num start
    if code.len() > 0 && (
        code.chars().nth(0).unwrap().is_alphabetic() || code.chars().nth(0).unwrap() == '_'
    ) && code.chars().nth(0).unwrap() != 'j' {
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
        let skip_ws = parse_whitespace(code.split_at(i).1);
        i += skip_ws.new_start;
        Some(ParseResult {
            new_start: i,
            token: Token::Identifier(ident_str.clone())
        })
    } else {
        None    
    }
}

/* True helper functions */

// Get a specified string of characters
fn parse_word(word: &str, code: &str) -> Option<ParseResult> {
    if word.len() <= code.len() {
        let mut i = 0;
        while i < word.len() {
            if word.chars().nth(i).unwrap() != code.chars().nth(i).unwrap() {
                return None;
            }
            i += 1;
        }
        let skip_ws = parse_whitespace(code.split_at(i).1);
        i += skip_ws.new_start;
        Some(ParseResult {
            new_start: i,
            token: Token::Word(String::from(word))
        })
    } else {
        None        
    }
}

fn parse_whitespace(code: &str) -> ParseResult {
    let mut i = 0;
    while i < code.len() && code.chars().nth(i).unwrap().is_whitespace() {
        i += 1;
    }
    ParseResult {
        new_start: i,
        token: Token::Whitespace
    }
}

