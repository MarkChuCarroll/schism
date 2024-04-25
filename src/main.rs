#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(parser, "/parser/schism.rs");

mod ast;
mod errors;
mod lex;
mod twist;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
}
