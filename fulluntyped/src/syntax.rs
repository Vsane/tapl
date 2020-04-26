use std::borrow::Borrow;

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Context {
    contexts: Vec<String>
}

impl Context {
    pub fn new() -> Self {
        Context {
            contexts: vec![]
        }
    }

    pub fn pick_fresh_name(&self, x: &str) -> (Context, String) {
        let name = x.to_string();
        if self.contexts.contains(&name) {
            self.pick_fresh_name(&(name + "'"))
        } else {
            let mut new_con = (*self).clone();
            new_con.contexts.push(name.clone());
            (new_con, name)
        }
    }

    pub fn name_to_index(&self, name: &str) -> Option<isize> {
        match self.contexts.iter().position(|x| x == name) {
            Some(idx) => Some((self.contexts.len() - 1 - idx) as isize),
            None => None
        }
    }

    pub fn index_to_name(&self, idx: isize) -> Option<String> {
        let con_len = self.contexts.len();
        if idx < con_len as isize {
            Some(self.contexts[con_len - 1 - idx as usize].clone())
        } else {
            None
        }
    }

    pub fn len(&self) -> isize {
        self.contexts.len() as isize
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum Term {
    True,
    False,
    Zero,
    Number(u64),
    IsZero(Box<Term>),
    If(Box<Term>, Box<Term>, Option<Box<Term>>),
    Succ(Box<Term>),
    Pred(Box<Term>),

    TmVar(isize, isize),
    TmAbs(String, Box<Term>),
    TmApp(Box<Term>, Box<Term>)
}




fn shift_walk(d: isize, c: isize, t: &Term) -> Term {
    match t.to_owned() {
        Term::IsZero(num) => {
            Term::IsZero(num)
        }
        Term::True => Term::True,
        Term::False => Term::False,
        Term::Zero => Term::Zero,
        Term::Succ(num) => {
            Term::Succ(num)
        }
        Term::Pred(num) => {
            Term::Pred(num)
        }
        Term::TmVar(idx, n) => {
            if idx >= c {
                Term::TmVar(idx + d, n + d)
            } else {
                Term::TmVar(idx, n + d)
            }
        }
        Term::Number(num) => Term::Number(num),
        Term::If(term1, term2, term3) => {
            if let Some(term3) = term3 {
                Term::If(
                    Box::new(shift_walk(d, c, term1.as_ref())),
                    Box::new(shift_walk(d, c, term2.as_ref())),
                    Some(Box::new(shift_walk(d, c, term3.as_ref()))),
                )
            } else {
                Term::If(
                    Box::new(shift_walk(d, c, term1.as_ref())),
                    Box::new(shift_walk(d, c, term2.as_ref())),
                    None,
                )
            }
        }
        Term::TmAbs(name , term) => {
            Term::TmAbs(name.clone(), Box::new(shift_walk(d, c + 1, term.as_ref())))
        }
        Term::TmApp(term1, term2) => {
            Term::TmApp(Box::new(shift_walk(d, c, term1.as_ref())), Box::new(shift_walk(d, c, term2.as_ref())))
        }
    }
}

fn subst_walk(j : isize, s: &Term, c: isize, t: &Term) -> Term {
    match t.to_owned() {
        Term::IsZero(num) => {
            Term::IsZero(num)
        }
        Term::True => Term::True,
        Term::False => Term::False,
        Term::Zero => Term::Zero,
        Term::Succ(num) => {
            Term::Succ(num)
        }
        Term::Pred(num) => {
            Term::Pred(num)
        }
        Term::TmVar(idx, n) => {
            if idx == j + c {
                shift(c, s)
            } else {
                Term::TmVar(idx, n)
            }
        }
        Term::Number(num) => Term::Number(num),
        Term::If(term1, term2, term3) => {
            if let Some(term3) = term3 {
                Term::If(
                    Box::new(subst_walk(j, s,c,  term1.as_ref())),
                    Box::new(subst_walk(j, s,c, term2.as_ref())),
                    Some(Box::new(subst_walk(j, s,c,  term3.as_ref()))),
                )
            } else {
                Term::If(
                    Box::new(subst_walk(j, s,c, term1.as_ref())),
                    Box::new(subst_walk(j, s,c, term2.as_ref())),
                    None,
                )
            }
        }
        Term::TmAbs(name, term) => Term::TmAbs(name, Box::new(subst_walk(j, s, c + 1, term.as_ref()))),
        Term::TmApp(term1, term2) => Term::TmApp(
            Box::new(subst_walk(j, s, c, term1.as_ref())),
            Box::new(subst_walk(j, s, c, term2.as_ref())),
        ),
    }
}

fn shift(d: isize, t: &Term) -> Term {
    shift_walk(d, 0, t)
}

fn subst(j : isize, s: &Term, t: &Term) -> Term {
    subst_walk(j, s, 0, t)
}

pub fn is_val(t: &Term) -> bool {
    match t {
        &Term::Number(_) => true,
        &Term::TmAbs(_, _) => true,
        &Term::True | &Term::False => true,
        _ => false,
    }
}

pub fn term_subst_top(s: &Term, t: &Term) -> Term {
    shift(-1, subst(0, shift(1, s).borrow(), t).borrow())
}

#[derive(Debug)]
pub enum EvalError {
    NoRule(Term),
    NoElse(Term)
}
