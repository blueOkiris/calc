/*
 * Author: Dylan Turner
 * Description: Entry point for calc
 */

mod args;
mod parser;

use args::read_args;
use parser::{
    Lexer, InputStream, RawToken
};

fn main() {
    match read_args() {
        Some(exec_str) => {
            // TODO: Run single line
        }, None => {
            // TODO: Set up repl and run each line
            let mut lexer = Lexer::new_with(
                &InputStream::from(&String::from("\\a:=log_10(110.3+57/6)"))
            );
            let mut token = lexer.read().unwrap();
            while token != RawToken::Eol {
                println!("Token: {:?}", token);
                token = lexer.read().unwrap();
            }
        }
    }
}
