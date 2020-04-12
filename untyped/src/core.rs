use std::borrow::Borrow;
use std::fmt;

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

struct ContextTerm<'a> {
    context: &'a Context,
    term: &'a Term,
}

impl<'a>  ContextTerm<'a> {
    fn new(ctx: &'a Context, t: &'a Term) -> Self {
        ContextTerm {
            context: ctx,
            term: t
        }
    }
}

impl<'a> fmt::Display for ContextTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ContextTerm{ context, term} = *self;
        match term.to_owned() {
            Term::TmAbs(ref name,ref t1 ) => {
                let (ctx1, name) = context.pick_fresh_name(name.as_ref());
                write!(f, "(λ {}. {})", name, ContextTerm::new(&ctx1, t1))
            }
            Term::TmApp(t1, t2) => {
                write!(f, "({} {})", ContextTerm::new(context, t1.as_ref()), ContextTerm::new(context, t2.as_ref()))
            }
            Term::TmVar(idx, _) => {
                write!(f, "{}", context.index_to_name(idx).unwrap())
            }
        }
    }
}


#[derive(Clone, PartialEq)]
pub enum Term {
    TmVar(isize, isize),
    TmAbs(String, Box<Term>),
    TmApp(Box<Term>, Box<Term>)
}

impl std::fmt::Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", ContextTerm::new(&Context::new(), self))
    }
}

fn shift_walk(d: isize, c: isize, t: &Term) -> Term {
    match t.to_owned() {
        Term::TmVar(idx, n) => {
            if idx >= c {
                Term::TmVar(idx + d, n + d)
            } else {
                Term::TmVar(idx, n + d)
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

fn shift(d: isize, t: &Term) -> Term {
    shift_walk(d, 0, t)
}

fn subst_walk(j : isize, s: &Term, c: isize, t: &Term) -> Term {

    match t.to_owned() {
        Term::TmVar(idx, n) => {
            if idx == j + c {
                shift(c, s)
            } else {
                Term::TmVar(idx, n)
            }
        }
        Term::TmAbs(name, term) => Term::TmAbs(name, Box::new(subst_walk(j, s, c + 1, term.as_ref()))),
        Term::TmApp(term1, term2) => Term::TmApp(
            Box::new(subst_walk(j, s, c, term1.as_ref())),
            Box::new(subst_walk(j, s, c, term2.as_ref())),
        )
    }
}

fn subst(j : isize, s: &Term, t: &Term) -> Term {
    subst_walk(j, s, 0, t)
}

fn is_val(t: &Term) -> bool {
    match t {
        &Term::TmAbs(_, _) => true,
        _ => false,
    }
}

fn term_subst_top(s: &Term, t: &Term) -> Term {
    shift(-1, subst(0, shift(1, s).borrow(), t).borrow())
}

#[derive(Debug)]
pub enum EvalError {
    NoRule(Term),
    NoElse(Term)
}

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
}

pub fn eval(t: &Term, ctx: &Context) -> Result<Term, EvalError> {
    match eval1(t, ctx) {
        Ok(t) => eval(t.borrow(), ctx),
        _ => Ok(t.to_owned())
    }
}