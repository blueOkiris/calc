# Calc

## Description

A powerful calculator in a Linux terminal

## Usage

`cargo run --release`

You will be prompted for expressions in a repl.

Enter the expressions and press enter to get results back.

Alternatively, run a single line via `cargo run --release -- --exec=<expr>`

Supports:
- float expressions (default) and int expressions (start a line with \\)
- +, -, /, *, % (mod; if in int mode)
- functions: `sqrt(`...`)`, `^(`...`)`, `log_`...`(`...`)`, `(` and `)`, `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- complex math via `j(`...`)` for imaginary
- assignments (only letters) `[a-z]:=<expr>`
- reusing last line with `$`
- everything uses radians
