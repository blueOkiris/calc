/*
 * Author: Dylan Turner
 * Description: Entry point for calc
 */

mod parser;
mod var;
mod eval;
mod args;
mod builtin;
mod complex;
mod plugin;

use std::{
    path::Path,
    fs::File,
    process::exit,
    io::{
        stdout, stdin, Write, BufRead, BufReader
    }
};
use dirs::config_dir;
use termion::{
    input::TermRead,
    event::Key,
    clear::CurrentLine
};
use crate::{
    parser::parse_stmt,
    args::cli_args,
    eval::{
        eval, Environment
    }
};

fn main() {
    let mut env = Environment::new();

    // Load the init file
    let conf = config_dir();
    if conf.is_some() {
        let mut init_file = conf.unwrap();
        init_file.push("calc");
        init_file.push("init");
        if Path::new(&init_file).exists() {
            let file = File::open(init_file);
            if file.is_ok() {
                for line in BufReader::new(file.unwrap()).lines() {
                    let stmt = parse_stmt(&line.unwrap());
                    match stmt {
                        Err(err) => println!("Init File Error: {}", err),
                        Ok(ast) => {
                            // Only print errors
                            let res = eval(&ast, &mut env);
                            if res.starts_with("Error") {
                                println!("Init File {}", res);
                            }
                        }
                    }       
                }
            }
        }
    }

    let args = cli_args();
    if args.is_present("stmts") {
        let lines = if args.value_of("stmts").unwrap() != "-" {
            args.value_of("stmts").unwrap().split('\n')
                .map(|s| String::from(s)).collect::<Vec<String>>()
        } else {
            let mut ls = Vec::new();
            let inp = stdin();
            for line in inp.lock().lines() {
                ls.push(line.unwrap().clone());
            }
            ls
        };
        for line in lines {
            let stmt = parse_stmt(line.as_str());
            match stmt {
                Err(err) => println!("Error: {}", err),
                Ok(ast) => println!("{}", eval(&ast, &mut env))
            }
        }
    } else {
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
            match stmt {
                Err(err) => writeln!(out, "Error: {}", err).unwrap(),
                Ok(ast) => writeln!(out, "{}", eval(&ast, &mut env)).unwrap()
            }
        }
    }
}

