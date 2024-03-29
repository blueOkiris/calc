# Calc

## Description

A simple yet powerful calculator in a Linux terminal

NOTE: This project is under development, so features are not implemented.

### Motivation

GUI calculators are fine, and many have fairly advanced keyboard functionality, but no matter which I try, I always find the experience fairly clunky. You can also do some shell math via `$((<expr>))`, but even then, I always seem to get slowed down. There are various programming/scripting options, but it's never just pass in the expression and give me a value.

I always get slowed down. When I want a calculator, I want something I can quickly switch to, get my response(s), and quit. My best option is honestly using a physical calc, but I don't always have that on me, so I've decided to make my own software solution.

That gives leads me to two design options:

1. Type a single character (I've chosen 'c') followed by an expression
2. A simple REPL experience.

There is some software which can provide such an experience, such as Maple for option 2, but as far as I could find, there was no free, open source, nor free and open source option, so that's my goal.

### Features

I needed it to support basic floating point math but also scientific options, integer math, and complex numbers all while not slowing me down to use, so I've implemented the necessary tools to do that as well as extensibility via persistent user defined functions and variables which go in the `.config/calc/init` file.
### Installation

Install by cloning this repo and running `cargo install --path .` from within it.

Requires:
- `cargo` the [Rust build system](https://www.rust-lang.org/tools/install)

Probably doesn't work on Windows, but the Windows terminal isn't worth using, so I don't really care

## Usage

Assuming you've installed it correctly, a program called `c` should be in your path.

From there, you can get the result of a single expression via `c <expr>` or `echo <expr> | c -`, or you enter a REPL by just typing `c`.

### Statements

Each line in the REPL or passed in directly can be one of three things:

1. An expr, e.g. `$ c 2+2` which prints `4`
2. A function definition, e.g:
    
    ```
    $ c
    CLI Calculator v1. Enter 'q' or press Ctrl-C to exit
    > \inc(x)->x+1
    > 3+inc(x)
    4
    >
    ```
    
    As noted before, you can store these custom functions as lines inside the `.config/calc/init`

3. A variable assignment, e.g:
    
    ```
    $ c
    CLI Calculator v1. Enter 'q' or press Ctrl-C to exit
    > let x := 7
    > x-3
    4
    >
    ```
    
    These can also be placed into `.config/calc/init`

### Data Types

The basic form of data is floats as that is typically the use case of a calculator, but a handful of others exist.

If you would like to do integer math, you can use integers by giving an integer value followed by '\_'

You can also make complex numbers either by using the function `comp(radius, angle)` or by adding `j` in front of the complex part of a float.

There are also lists, defined like `[ #1, #2, #3, ... ]`

All functions and operations work on all data types, but there are some consequences like floats and integers turning into each other or applying options to every member of a list, so be somewhat careful when doing complex operations.

### Built-in Functions

There are a number of functions to expand the functionality of the calculator to work in different situations. They are listed here:

| Function | Description |
|:----:|:-----------:|
| call(lib, args...) | Calls a function from a dynamic library. It is a special function that takes an identifier as input (lib name) as well as a list. See the following section for how to make a lib |
| sin(x) | Return sine(x) where x (and everything else) is radians |
| cos(x) | cosine |
| tan(x) | tangent |
| asin(x) | arcsine |
| acos(x) | arccosine |
| atan(x) | arctan |
| d2r(x) | degrees to radians |
| r2d(x) | radians to degrees |
| log(x) | log base 10 of x |
| ln(x) | log base e of x |
| e() | e |
| pi() | π |
| mod(x, y) | modulus of two ints (will truncate floats) |
| floor(x) | floor |
| ceil(x) | ceiling |
| abs(x) | \|x\| |
| idx(ls, n) | Access the nth item in ls |
| len(ls) | Length of ls |
| app(ls, i) | append i to the end of ls |
| del(ls, n) | remove the nth item in ls |
| sign(x) | 0 for 0, -1 for neg, 1 for pos |
| comp(r, ang) | build a complex number from radius and angle |

NOT IMPLEMENTED YET

#### Custom Built-Ins

If you want to make a custom built-in function, you'll need to make a Rust library and build it as a dynamic library.

At a minimum, your Cargo.toml should look like:

```
[package]
name = "<package_name>"
version = "<version>"
edition = "2021"

[dependencies]
...

[lib]
name = "<package_name>"
crate-type = [ "cdylib" ]
path = "src/<main code file>"
```

and your main code file should look like

```
#[derive(Clone, Copy)]
pub struct IComplex {
    pub len: i64,
    pub angle_deg: i64
}

#[derive(Clone, Copy)]
pub struct FComplex {
    pub len: f64,
    pub angle: f64
}

#[derive(Clone)]
pub struct Var {
    pub ls_data: Option<Vec<Var>>,
    pub num_data: Option<FComplex>,
    pub int_data: Option<IComplex>
}

#[no_mangle]
pub fn execute(vars: &Vec<Var>) -> Var {
    Var {
        ls_data: None,
        num_data: None,
        int_data: None
    }
}
```

There is an example plugin under the ex-plug/ folder

