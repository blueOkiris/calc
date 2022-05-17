/*
 * Author: Dylan Turner
 * Description: Entry point for calc
 */

mod args;
mod parser;

use args::read_args;

fn main() {
    match read_args() {
        Some(exec_str) => {
            // TODO: Run single line
        }, None => {
            // TODO: Set up repl and run each line
        }
    }
}
