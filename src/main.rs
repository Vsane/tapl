use arith::parser::{Lexer, Parser};
use arith::arith::eval1;

fn main() {
    let tmp_str = "if (iszero (succ zero)) then true else succ zero";
    //let tmp_str = "iszero zero";

    let mut lex = Lexer::new(tmp_str.chars());
    lex.lex_input();
    println!("{:?}", lex.result);

    let mut parser = Parser::new(lex.result.into_iter());
    let out = parser.expr();
    println!("{:?}", eval1(out.unwrap()));
}
