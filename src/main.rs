/*
 * Author: Dylan Turner
 * Description: Entry point for calc
 */

mod parser;
mod tests;
mod args;

use parser::parse_stmt;

use crate::args::cli_args;

fn main() {
    let args = cli_args();
    if args.is_present("stmts") {
        let stmt = parse_stmt(args.value_of("stmts").unwrap());
        println!("Parsed {:?}", stmt.unwrap().token);
        // TODO: Immediately evaluate
    } else {
        // TODO: Enter REPL
    }
}

