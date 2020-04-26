use std::borrow::Borrow;
use std::fmt;
use crate::core::Type::TyArr;

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    TyArr(Box<Type>, Box<Type>),
    TyBool,
}


impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::TyArr(ref ty1, ref ty2) => write!(f, "({} -> {})", ty1, ty2),
            Type::TyBool => write!(f, "Bool"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Context {
    contexts: Vec<(String, Type)>
}

impl Context {
    pub fn new() -> Self {
        Context {
            contexts: vec![]
        }
    }

    pub fn pick_fresh_name(&self, x: &str, ty: Type) -> (Context, String) {
        let name = x.to_string();
        if self.contexts.iter().find(|(name, _ty)| x == name).is_some() {
            self.pick_fresh_name(&(x.to_string() + "'"), ty)
        } else {
            let mut new_con = (*self).clone();
            new_con.contexts.push((x.parse().unwrap(), ty));
            (new_con, name)
        }
    }

    pub fn name_to_index(&self, name: &str) -> Option<isize> {
        match self.contexts.iter().position(|(x, _ty)| x == name) {
            Some(idx) => Some((self.contexts.len() - 1 - idx) as isize),
            None => None
        }
    }

    pub fn index_to_name(&self, idx: isize) -> Option<String> {
        let con_len = self.contexts.len();
        if idx < con_len as isize {
            Some(self.contexts[con_len - 1 - idx as usize].0.clone())
        } else {
            None
        }
    }

    pub fn get_type(&self, idx: isize) -> Option<Type> {
        let con_len = self.contexts.len();
        if idx < con_len as isize {
            Some(self.contexts[con_len - 1 - idx as usize].1.clone())
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
    TmVar(isize, isize),
    TmAbs(String, Type, Box<Term>),
    TmApp(Box<Term>, Box<Term>),
    TmTrue,
    TmFalse,
    TmIf(Box<Term>, Box<Term>, Box<Term>)
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ContextTerm::new(&Context::new(), self))
    }
}

pub struct ContextTerm<'a> {
    context: &'a Context,
    term: &'a Term,
}

impl<'a>  ContextTerm<'a> {
    pub fn new(ctx: &'a Context, t: &'a Term) -> Self {
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
            Term::TmAbs(ref x, ref ty, ref term ) => {
                let (ctx1, name) = context.pick_fresh_name(x, ty.to_owned());
                write!(f, "(λ {}: {}. {})", name, ty, ContextTerm::new(&ctx1, term))
            }
            Term::TmApp(t1, t2) => {
                write!(f, "({} {})", ContextTerm::new(context, t1.as_ref()), ContextTerm::new(context, t2.as_ref()))
            }
            Term::TmVar(idx, _) => {
                write!(f, "{}", context.index_to_name(idx).unwrap())
            }
            Term::TmTrue => write!(f, "true"),
            Term::TmFalse => write!(f, "false"),
            Term::TmIf(ref t1, ref t2, ref t3) => write!(f, "(if {} {} {})", ContextTerm::new(context, t1), ContextTerm::new(context, t2), ContextTerm::new(context, t3)),
        }
    }
}


#[derive(Clone, PartialEq, Debug)]
pub enum TypeError {
    TyErr,
}


pub fn type_of(t: &Term, ctx: &Context) -> Result<Type, TypeError> {
    match t.to_owned() {
        Term::TmVar(id, _) => Ok(ctx.get_type(id).unwrap()),
        Term::TmApp(term1, term2) => {
            let ty1 = type_of(term1.as_ref(), ctx)?;
            let ty2 = type_of(term2.as_ref(), ctx)?;
            match ty1 {
                Type::TyArr(type1, type2) if ty2 == *type1 => Ok(*type2),
                _ => Err(TypeError::TyErr),
            }
        }
        Term::TmAbs(name, ty, term1) => {
            let (ctx1, _name) = ctx.pick_fresh_name(name.as_ref(), ty.clone());
            Ok(TyArr(Box::new(ty), Box::new(type_of(term1.as_ref(), ctx1.borrow())?)))
        }
        Term::TmTrue | Term::TmFalse => Ok(Type::TyBool),
        Term::TmIf(ref t1, ref t2, ref t3) => {
            match type_of(t1, ctx)? {
                Type::TyBool => {
                    let ty2 = type_of(t2, ctx)?;
                    let ty3 = type_of(t3, ctx)?;
                    if ty2 == ty3 {
                        Ok(ty2)
                    } else {
                        Err(TypeError::TyErr)
                    }
                }
                _ => Err(TypeError::TyErr)
            }
        }
    }
}

