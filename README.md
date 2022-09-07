# Calc

## Description

A simple yet powerful calculator in a Linux terminal

### Motivation

GUI calculators are fine, and many have fairly advanced keyboard functionality, but no matter which I try, I always find the experience fairly clunky. You can also do some shell math via `$((<expr>))`, but even then, I always seem to get slowed down.

When I want a calculator, I want something I can quickly switch to, get my response(s), and quit.

That gives leads me to two design options:

1. Type a single character (I've chosen '%') followed by an expression
2. A simple REPL experience.

There is some software which can provide such an experience, such as Maple for option 2, but as far as I could find, there was no free, open source, nor free and open source option, so that's my goal.

### Features

I needed it to support basic floating point math but also scientific options, integer math, and complex numbers all while not slowing me down to use, so I've implemented the necessary tools to do that as well as extensibility via user defined functions which go in the `.config/calc` file.

## Usage

Run `cargo build --release`. The executable '%' will be located in the created target/release/ folder.

From there, you can either get the result of a single expression via `c <expr>` or enter a REPL by just typing `c`.



