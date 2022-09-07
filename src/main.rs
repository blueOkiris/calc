/*
 * Author: Dylan Turner
 * Description: Entry point for calc
 */

mod parser;
mod tests;
mod args;

use std::{
    process::exit,
    io::{
        stdout, stdin, Write
    }
};
use termion::{
    input::TermRead,
    event::Key,
    clear::CurrentLine
};
use crate::{
    parser::parse_stmt,
    args::cli_args
};

fn main() {
    // let mut env - TODO: Interpreter environment implementation
    let args = cli_args();
    if args.is_present("stmts") {
        let lines = args.value_of("stmts").unwrap().split('\n').collect::<Vec<&str>>();
        for line in lines {
            let stmt = parse_stmt(line);
            println!("Parsed {:?}", stmt.unwrap().token);
            // TODO: Immediately evaluate
        }
    } else {
        // TODO: Enter REPL
        let mut out = stdout();

        writeln!(
            out, "{}CLI Calculator v1. Enter 'q' or press Ctrl-C to exit", CurrentLine
        ).expect("Could not write to stdout");
        loop {
            write!(out, "> ").expect("Could not write to stdout");
            out.flush().unwrap();
            let mut line = String::new();
            let inp = stdin();

            for c in inp.keys() {
                match c.unwrap() {
                    Key::Char('\n') => break,
                    Key::Char('q') | Key::Ctrl('c') => exit(0),
                    Key::Char(c) => {
                        line.push(c)
                    }, _ => {}
                }
            }

            let stmt = parse_stmt(line.as_str());
            writeln!(out, "Parsed {:?}", stmt.unwrap().token).unwrap();
            // TODO: Immediately evaluate
        }
    }
}

