use crate::{ast::ASTParser, interpreter::Evaluator, token::Tokenizer};

mod ast;
mod interpreter;
mod token;

fn main() {
    let example = "(println \"Hello, world\")";

    let mut tknz = Tokenizer::new(example.to_string());
    if let Err(e) = tknz.try_parse_all() {
        println!("{:?}", (tknz.location, e));
    } else {
        let mut parser = ASTParser::new(tknz.tokens);
        match parser.try_parse_all() {
            Ok(()) => {
                println!("{:?}", parser.roots);
                let mut eval = Evaluator::new(parser.roots);
                let expr = eval.try_interpret_next().unwrap();
                eval.evaluate_expr(expr);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
