use crate::arith::Term::Succ;
use std::borrow::Borrow;
use crate::arith::EvalError::{NoRule, NoElse};
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Term {
    True,
    False,
    Zero,
    IsZero(Box<Term>),
    If(Box<Term>, Box<Term>, Option<Box<Term>>),
    Succ(Box<Term>),
    Pred(Box<Term>),
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Term::True => write!(f, "True"),
            Term::False => write!(f, "False"),
            Term::If(ref t0, ref t1, ref t2) => write!(f, "(If {:?} {:?} {:?})", t0, t1, t2.as_ref().unwrap()),
            Term::Zero => write!(f, "Zero"),
            Term::Succ(ref t0) => write!(f, "(Succ {:?})", t0),
            Term::Pred(ref t0) => write!(f, "(Pred {:?})", t0),
            Term::IsZero(ref t0) => write!(f, "(IsZero {:?})", t0),
        }
    }
}

pub enum EvalError {
    NoRule(Term),
    NoElse(Term)
}


impl fmt::Debug for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            NoRule(term) => write!(f, "[*] No rule for {:?}", term),
            NoElse(term) => write!(f, "[*] No else for {:?}", term),
        }
    }
}

fn is_numeric_val(t: &Term) -> bool {
    match t.clone() {
        Term::Zero => true,
        Succ(ref term1) => is_numeric_val((*term1).as_ref()),
        _ => false,
    }
}

pub fn eval1(term: Term) -> Result<Term, EvalError> {
    match term {
        Term::True => Ok(Term::True),
        Term::False => Ok(Term::False),
        Term::Zero => Ok(Term::Zero),
        Term::If(term1, term2, term3) => {
            match *term1 {
                Term::True => {
                    Ok(eval1(*term2)?)
                }
                Term::False => {
                    if term3 != None {
                        Ok(eval1(*(term3.unwrap()))?)
                    } else {
                        Err(EvalError::NoElse(*term1))
                    }
                }
                _ => {
                    let tmp_term = Term::If(Box::new(eval1(*term1)?), term2, term3);
                    Ok(eval1(tmp_term)?)
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
                    let tmp_term = Term::IsZero(Box::from(eval1(*term1)?));
                    Ok(eval1(tmp_term)?)
                }
            }
        }
        Term::Pred(term1) => {
            match *term1 {
                Term::Zero => {
                    Ok(Term::Zero)
                }
                Term::Succ(term2) if is_numeric_val((*term2).borrow())  => {
                    Ok(eval1(*term2)?)
                }
                _ => {
                    let tmp_term = Term::Pred(Box::from(eval1(*term1)?));
                    Ok(eval1(tmp_term)?)
                }
            }

        }
        Term::Succ(term1) => {
            let tmp_term = Term::Succ(Box::from(eval1(*term1)?));
            Ok(tmp_term)
        }
    }
}