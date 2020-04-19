use crate::syntax::{Context, Term, term_subst_top, EvalError, is_val};
use std::borrow::Borrow;
use crate::syntax::Term::Succ;

fn is_numeric_val(t: &Term) -> bool {
    match t.clone() {
        Term::Zero => true,
        Succ(ref term1) => is_numeric_val((*term1).as_ref()),
        _ => false,
    }
}

fn eval1(t: &Term, ctx: &Context) -> Result<Term, EvalError> {
    match t.to_owned() {
        Term::If(term1, term2, term3) => {
            match *term1 {
                Term::True => {
                    Ok(*term2)
                }
                Term::False => {
                    if term3 != None {
                        Ok(*(term3.unwrap()))
                    } else {
                        Err(EvalError::NoElse(*term1))
                    }
                }
                _ => {
                    let tmp_term = Term::If(Box::new(eval1((*term1).borrow(), ctx)?), term2, term3);
                    Ok(tmp_term)
                }
            }
        }
        Term::IsZero(term1) => {
            match *term1 {
                Term::Zero => {
                    Ok(Term::True)
                }
                Term::Succ(term2) if is_numeric_val((*term2).borrow()) => {
                    Ok(Term::False)
                }
                _ => {
                    let tmp_term = Term::IsZero(Box::from(eval1((*term1).borrow(), ctx)?));
                    Ok(tmp_term)
                }
            }
        }
        Term::Pred(term1) => {
            match *term1 {
                Term::Zero => {
                    Ok(Term::Zero)
                }
                Term::Succ(term2) if is_numeric_val((*term2).borrow())  => {
                    Ok(*term2)
                }
                _ => {
                    let tmp_term = Term::Pred(Box::from(eval1((*term1).borrow(), ctx)?));
                    Ok(tmp_term)
                }
            }
        }
        Term::Succ(term1) => {
            match *term1 {
                Term::Pred(term2) => Ok(Term::Succ(Box::from(eval1(Term::Pred(term2).borrow(), ctx).unwrap()))),
                _ => Err(EvalError::NoRule(*term1))
            }
        }
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
    //unimplemented!()
}

pub fn eval(t: &Term, ctx: &Context) -> Result<Term, EvalError> {
    match eval1(t, ctx) {
        Ok(t) => eval(t.borrow(), ctx),
        _ => Ok(t.to_owned())
    }
}