use untyped::parser::{Lexer, Parser};
use std::borrow::Borrow;
use untyped::core::eval;

fn main() {
    let tmp_str = "((λ x.(λ y. (y x))) (λ x. x)) (λ x.(λ y. (y x)))";
    let mut lex = Lexer::new(tmp_str.chars());
    lex.lex_input();

    let mut parser = Parser::new(lex.result.into_iter());
    let out = parser.parse();
    println!("{:?}\n", eval(out.unwrap().borrow(), parser.ctx.borrow()));

}

