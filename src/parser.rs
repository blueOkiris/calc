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
 * <expr> ::=       <factor> | <func> | '(' <expr> ')' | '-' <expr> | '\' <expr> | /[a-z]/ ":=" <expr>
 * 
 * <keyword> ::=    '^' | "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "j"
 * <func> ::=       (<keyword> | "log_" /[0-9]+/) '(' <expr> ')'
 */

const KEYWORDS: [&'static str; 9] = [
    "^", "sqrt", "sin", "cos", "tan", "asin", "acos", "atan", "j"
];

/* Representation of ebnf tokens */

#[derive(Clone, PartialEq, Debug)]
pub enum RawToken {
    Num(f64),
    TermSymbol(char),
    FactorSymbol(char),
    LeftParenth,
    RightParenth,
    IntOp,
    AssignOp,
    Keyword(String),
    Log(usize),
    Letter(char),
    Eol
}

#[derive(Clone, Debug)]
pub struct Term {
    pub num: f64,
    pub term_expr: Option<(char, f64)>
}

#[derive(Clone, Debug)]
pub struct Factor {
    pub term: Term,
    pub factor_expr: Option<(char, Term)>
}

#[derive(Clone, Debug)]
pub struct Function {
    pub keyword: Option<String>,
    pub log: Option<usize>,

}

#[derive(Clone, Debug)]
pub struct Expr {
    pub factor: Option<Factor>,
    pub func: Option<Function>
}

// Lex the input into tokens for the parser
pub struct Lexer {
    inp: InputStream
}

impl Lexer {
    pub fn new_with(inp: &InputStream) -> Self {
        Self {
            inp: inp.clone()
        }
    }

    pub fn _peek(&mut self) -> Result<RawToken, String> {
        let stored_inp = self.inp.clone();
        let next_token = self.read();
        self.inp = stored_inp; // Don't actually move forward
        next_token
    }

    pub fn read(&mut self) -> Result<RawToken, String> {
        if !self.inp.not_eof() {
            return Ok(RawToken::Eol);
        }

        let mut c = self.inp.peek();

        // Skip whitespace
        while char::is_whitespace(c) {
            self.inp.read();
            c = self.inp.peek();
        }

        if char::is_numeric(c) { // Parse a number
            self.lex_number()
        } else if c == '+' || c == '-' {
            self.inp.read();
            Ok(RawToken::TermSymbol(c))
        } else if c == '*' || c == '/' || c == '%' {
            self.inp.read();
            Ok(RawToken::FactorSymbol(c))
        } else if c == '(' {
            self.inp.read();
            Ok(RawToken::LeftParenth)
        } else if c == ')' {
            self.inp.read();
            Ok(RawToken::RightParenth)
        } else if c == '\\' {
            self.inp.read();
            Ok(RawToken::IntOp)
        } else if c == ':' {
            self.inp.read();
            let c = self.inp.read();
            if c != '=' {
                return Err(format!(
                    "Unexpected character '{}' on line {}, col {}!",
                    c, self.inp.line, self.inp.col - 1
                ));
            }
            Ok(RawToken::AssignOp)
        } else if c == 'l' { // Try parsing log_# but if not, put l down as letter
            let stored_inp = self.inp.clone();
            match self.lex_log() {
                Ok(token) => Ok(token),
                Err(_) => {
                    self.inp = stored_inp;
                    self.inp.read();
                    Ok(RawToken::Letter('l'))
                }
            }
        } else if char::is_alphabetic(c) { // Try getting a keyword, else just letter
            let stored_inp = self.inp.clone();
            for keyword in KEYWORDS {
                let mut success = true;
                c = self.inp.read();
                for letter in keyword.chars() {
                    if c != letter {
                        self.inp = stored_inp.clone(); // Reset for the next one
                        success = false;
                        break;
                    }
                    c = self.inp.read();
                }
                if success {
                    return Ok(RawToken::Keyword(String::from(keyword)));
                }
            }
            self.inp.read();
            Ok(RawToken::Letter(c))
        } else {
            Err(format!(
                "Unexpected character '{}' on line {}, col {}!", c, self.inp.line, self.inp.col
            ))
        }
    }

    fn lex_log(&mut self) -> Result<RawToken, String> {
        self.inp.read(); // Only called if we already know first is 'l'
        if self.inp.peek() != 'o' {
            return Err(format!(
                "Expected 'o' for log on line {}, col {}", self.inp.line, self.inp.col
            ));
        }
        self.inp.read();
        if self.inp.peek() != 'g' {
            return Err(format!(
                "Expected 'g' for log on line {}, col {}", self.inp.line, self.inp.col
            ));
        }
        self.inp.read();
        if self.inp.peek() != '_' {
            return Err(format!(
                "Expected 'o' for log on line {}, col {}", self.inp.line, self.inp.col
            ));
        }
        self.inp.read();
        if !char::is_numeric(self.inp.peek()) {
            return Err(format!(
                "Expected number after 'log_' on line {}, col {}", self.inp.line, self.inp.col
            ));
        }
        let mut num_str = String::new();
        while char::is_numeric(self.inp.peek()) {
            num_str.push(self.inp.read());
        }
        match num_str.parse::<usize>() {
            Err(_) => Err(format!(
                "Failed to parse float '{}' at line {}, col {}",
                num_str, self.inp.line, self.inp.col
            )), Ok(val) => Ok(RawToken::Log(val))
        }
    }

    fn lex_number(&mut self) -> Result<RawToken, String> {
        let start_line = self.inp.line;
        let start_col = self.inp.col;

        // Get the first collection of digits
        let mut num_str = String::new();
        while char::is_numeric(self.inp.peek()) {
            num_str.push(self.inp.read());
        }

        // Then look for /\.[0-9]+/
        if self.inp.peek() == '.' {
            num_str.push(self.inp.read());
            while char::is_numeric(self.inp.peek()) {
                num_str.push(self.inp.read());
            }
        }

        // Then parse it into an actual float to return
        match num_str.parse::<f64>() {
            Err(_) => Err(format!(
                "Failed to parse float '{}' at line {}, col {}", num_str, start_line, start_col
            )), Ok(val) => Ok(RawToken::Num(val))
        }
    }
}

// Tool for easily reading in the input data
#[derive(Clone, Debug)]
pub struct InputStream {
    expr_str: String,
    index: usize,

    pub line: usize,
    pub col: usize
}

impl InputStream {
    pub fn from(expr: &String) -> Self {
        Self {
            expr_str: expr.clone(),
            index: 0,
            line: 1,
            col: 1
        }
    }

    pub fn peek(&self) -> char {
        if self.index < self.expr_str.len() {
            self.expr_str.chars().nth(self.index).unwrap()
        } else {
            '\0'
        }
    }

    pub fn read(&mut self) -> char {
        if self.index < self.expr_str.len() {
            // Keep track of line
            self.col += 1;
            if self.expr_str.chars().nth(self.index).unwrap() == '\n' {
                self.col = 1;
                self.line += 1;
            }

            let c = self.expr_str.chars().nth(self.index).unwrap();
            self.index += 1;
            c
        } else {
            '\0'
        }
    }

    pub fn not_eof(&self) -> bool {
        self.index < self.expr_str.len()
    }
}
