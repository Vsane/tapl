#[cfg(test)]
mod tests {
    use crate::parser::{Lexer, Parser};
    use crate::core::eval;
    use std::borrow::Borrow;
    use crate::core::Term::TmAbs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn eval_test() {
        let mut tmp_str = "(λ x. (λ y. y) x) (λ x. x)"; // (λ.0)

        let mut lex = Lexer::new(tmp_str.chars());
        lex.lex_input();
        let mut parser = Parser::new(lex.result.into_iter());
        let out = parser.parse();

        assert_eq!(format!("{:?}", eval(out.unwrap().borrow(), parser.ctx.borrow()).unwrap()), "(λ.0)");


        tmp_str = "(λ x. (x x)) (λ x. x)"; //(λ.0)

        lex = Lexer::new(tmp_str.chars());
        lex.lex_input();
        parser = Parser::new(lex.result.into_iter());
        let out = parser.parse();

        assert_eq!(format!("{:?}", eval(out.unwrap().borrow(), parser.ctx.borrow()).unwrap()), "(λ.0)");


        tmp_str = "((λ x.(λ y. (y x))) (λ x. x)) (λ x.(λ y. (y x)))"; // (λ.0)

        lex = Lexer::new(tmp_str.chars());
        lex.lex_input();
        parser = Parser::new(lex.result.into_iter());
        let out = parser.parse();

        assert_eq!(format!("{:?}", eval(out.unwrap().borrow(), parser.ctx.borrow()).unwrap()), "(λ.(0 (λ.0)))");
    }
}


pub mod parser;
pub mod core;
