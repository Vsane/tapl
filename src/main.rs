use arith::parser::{Lexer, Parser};
use arith::arith::eval;

fn main() {
    let tmp_str = "if (iszero (succ zero)) then true else zero";
    //let tmp_str = "iszero zero";

    let mut lex = Lexer::new(tmp_str.chars());
    lex.lex_input();

    let mut parser = Parser::new(lex.result.into_iter());
    let out = parser.expr();
    println!("{:?}", out);

    println!("{:?}", eval(out.unwrap()));
}
