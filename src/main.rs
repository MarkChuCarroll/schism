#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub schism_parser); // synthesized by LALRPOP

mod ast;
mod error;
mod lex;
mod twist;

#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
}
