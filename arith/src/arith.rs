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
            Term::If(ref term1, ref term2, ref term3) => write!(f, "(If {:?} {:?} {:?})", term1, term2, term3.as_ref().unwrap()),
            Term::Zero => write!(f, "Zero"),
            Term::Succ(ref term1) => write!(f, "(Succ {:?})", term1),
            Term::Pred(ref term1) => write!(f, "(Pred {:?})", term1),
            Term::IsZero(ref term1) => write!(f, "(IsZero {:?})", term1),
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

fn eval1(term: Term) -> Result<Term, EvalError> {
    match term {
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
                    let tmp_term = Term::If(Box::new(eval1(*term1)?), term2, term3);
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
                    let tmp_term = Term::IsZero(Box::from(eval1(*term1)?));
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
                    let tmp_term = Term::Pred(Box::from(eval1(*term1)?));
                    Ok(tmp_term)
                }
            }
        }
        Term::Succ(term1) => {
            match *term1 {
                Term::Pred(term2) => Ok(Term::Succ(Box::from(eval1(Term::Pred(term2)).unwrap()))),
                _ => Err(EvalError::NoRule(*term1))
            }
        }
        term1 => Err(EvalError::NoRule(term1))
    }
}

pub fn eval(term: Term) -> Term {
    match eval1(term.clone()) {
        Ok(term1) => {
            eval(term1)
        }
        Err(_) => term,
    }
}


fn is_val(t: &Term) -> bool {
    match t.clone() {
        Term::True | Term::False => true,
        ref t1 if is_numeric_val(t1) => true,
        _ => false,
    }
}


pub fn big_eval(term: Term) -> Term {
    if is_val(term.borrow()) {
        return term;
    }

    match term {
        Term::If(term1, term2, term3) => {
            match *term1 {
                Term::True => {
                    big_eval(*term2)
                }
                Term::False => {
                    if term3 != None {
                        big_eval(*term3.unwrap())
                    } else {
                        panic!("Err~~");
                    }
                }
                _ => {
                    let tmp_term = Term::If(Box::new(big_eval(*term1)), term2, term3);
                    big_eval(tmp_term)
                }
            }
        }
        Term::IsZero(term1) => {
            match *term1 {
                Term::Zero => {
                    Term::True
                }
                Term::Succ(term2) if is_numeric_val((*term2).borrow()) => {
                    Term::False
                }
                _ => {
                    let tmp_term = Term::IsZero(Box::from(big_eval(*term1)));
                    big_eval(tmp_term)
                }
            }
        }
        Term::Pred(term1) => {
            match *term1 {
                Term::Zero => {
                    Term::Zero
                }
                Term::Succ(term2) if is_numeric_val((*term2).borrow())  => {
                    big_eval(*term2)
                }
                _ => {
                    let tmp_term = Term::Pred(Box::from(big_eval(*term1)));
                    big_eval(tmp_term)
                }
            }
        }
        Term::Succ(term1) => {
            match *term1 {
                Term::Pred(term2) => Term::Succ(Box::from(big_eval(Term::Pred(term2)))),
                _ => *term1
            }
        }
        term1 => term1
    }
}


