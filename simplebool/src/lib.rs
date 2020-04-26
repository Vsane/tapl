#[cfg(test)]
mod tests {
    use crate::parser::{Lexer, Parser};
    use crate::core::{type_of, Context, TypeError};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    fn type_check() {
        let tmp_str = "if true then false else (λ x:Bool. x)";
        let mut lex = Lexer::new(tmp_str.chars());
        lex.lex_input();

        let mut parser = Parser::new(lex.result.into_iter());
        let out = parser.parse();
        assert_eq!(type_of(&out.unwrap(), &Context::new()), Err(TypeError::TyErr));
    }
}
pub mod parser;
pub mod core;