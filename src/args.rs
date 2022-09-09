/*
 * Author: Dylan Turner
 * Description: Implement a simple cli arg help menu
 */

use clap::{
    Arg, Command, crate_version, ArgMatches
};

pub fn cli_args() -> ArgMatches {
    Command::new("CLI Calculator")
        .version(crate_version!())
        .author("Dylan Turner <dylantdmt@gmail.com>")
        .about("A simple yet powerful cli calculator/programming language.")
        .arg(
            Arg::new("stmts")
                .takes_value(true)
                .help("Immediately evaluate '\\n' separated lines\n(leave blank for REPL)")
        ).get_matches()
}

