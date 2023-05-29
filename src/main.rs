#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub schism_parser); // synthesized by LALRPOP

mod ast;
mod error;
mod lex;
mod twist;

fn main() {
    println!("Hello, world!");
}
