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
                .short('s')
                .takes_value(true)
                .help("Immediately evaluate '\\n' separated lines\n(as opposed to REPL)")
        ).get_matches()
}

