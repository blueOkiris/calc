/*
 * Author: Dylan Turner
 * Description: Handle argument stuff and help
 */

use clap::{
    Arg, Command, crate_version
};

// Helper to get argmatches which will be parsed
pub fn read_args() -> Option<String> {
    let args = Command::new("basename")
        .version(crate_version!())
        .author("Dylan Turner <dylantdmt@gmail.com>")
        .long_about(
            "Powerful terminal calculator\n\
            See usage on github: https://github.com/blueOkiris/calc\n"
        ).arg(
            Arg::new("exec")
                .short('e')
                .long("exec")
                .takes_value(true)
                .help("Instead of repl, run just this string")
        ).get_matches();
    if args.is_present("exec") {
        Some(String::from(args.value_of("exec").unwrap()))
    } else {
        None
    }
}
