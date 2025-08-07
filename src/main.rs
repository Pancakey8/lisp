// Annoying, disabled for now
// TODO: Enable later
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{env, fs};

use crate::{ast::ASTParser, interpreter::Evaluator, token::Tokenizer};

mod ast;
mod interpreter;
mod lisp;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let example = fs::read_to_string(&args[1]).unwrap();

    let mut tknz = Tokenizer::new(example);
    if let Err(e) = tknz.try_parse_all() {
        println!("{:?}", (tknz.location, e));
    } else {
        let mut parser = ASTParser::new(tknz.tokens);
        match parser.try_parse_all() {
            Ok(()) => {
                let mut eval = Evaluator::new(parser.roots);
                match eval.run() {
                    Ok(()) => {}
                    Err(e) => println!("{:?}", e),
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
