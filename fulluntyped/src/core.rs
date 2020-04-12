use crate::syntax::{Context, Term, term_subst_top, EvalError, is_val};
use std::borrow::Borrow;

fn eval1(t: &Term, ctx: &Context) -> Result<Term, EvalError> {
    match t.to_owned() {
        Term::TmApp(term1, term2) if is_val(&term2) => {
            if let Term::TmAbs(_, t1) = *term1 {
                Ok(term_subst_top(term2.as_ref(), t1.as_ref()))
            } else {
                Ok(Term::TmApp(Box::new(eval1(term1.as_ref(), ctx)?), term2))
            }
        }
        Term::TmApp(term1, term2) if is_val(&term1) => {
            Ok(Term::TmApp(term1, Box::new(eval1(term2.as_ref(), ctx)?)))
        }
        Term::TmApp(term1, term2) => {
            Ok(Term::TmApp(Box::new(eval1(term1.as_ref(), ctx)?), term2))
        }
        _ => Err(EvalError::NoRule(t.to_owned()))
    }
    unimplemented!()
}

pub fn eval(t: &Term, ctx: &Context) -> Result<Term, EvalError> {
    match eval1(t, ctx) {
        Ok(t) => eval(t.borrow(), ctx),
        _ => Ok(t.to_owned())
    }
}