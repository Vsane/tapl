use crate::syntax::Term;
use crate::syntax::Term::{TmAbs, TmApp, Succ};
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use crate::core::eval;
use std::borrow::Borrow;
use std::collections::HashMap;

fn ret_term(tmp_str:&str) -> Term {
    let mut lex = Lexer::new(tmp_str.chars());
    lex.lex_input();

    let mut parser = Parser::new(lex.result.into_iter());
    let out = parser.parse();
    eval(out.unwrap().borrow(), parser.ctx.borrow()).unwrap()
}

pub fn suc_term() -> Term {
    ret_term("(λn.λf.λx.f (n f x))")
}

pub fn prd_term() -> Term {
    ret_term("(λn.λf.λx.n (λg.λh.h (g f)) (λu.x) (λu.u))")
}

pub fn plus_term() -> Term {
    ret_term("(λm.λn.λf.λx.m f (n f x))")
}

pub fn mult_term() -> Term {
    ret_term("(λm.λn.λf.m (n f))")
}

pub fn sub_term() -> Term {
    ret_term("(λm.λn.n pred m)")
}