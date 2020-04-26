use crate::syntax::{Context, Term};
use crate::lexer::Token;
use std::fmt::Debug;
use crate::builtin::{suc_term, prd_term, plus_term, sub_term, mult_term};

#[derive(Clone, PartialEq)]
pub struct Parser<T: Iterator<Item = Token> + Debug + Clone> {
    pub ctx: Context,
    token: T,
    tok0: Option<Token>,
    pub result: Option<Term>
}

impl<T> Parser<T>
    where
        T: Iterator<Item = Token> + Debug + Clone {
    pub fn new(input: T) -> Self {
        let mut parser = Parser {
            ctx: Context::new(),
            token: input,
            tok0: None,
            result: None
        };
        parser.next_token();

        parser
    }

    fn lambda(&mut self) -> Option<Term> {
        self.next_token(); //lambda
        let prev_ctx = self.ctx.clone();

        let (con_next, name) = match self.tok0.clone() {
            Some(Token::Var(var_name)) => {
                self.next_token();
                self.ctx.pick_fresh_name(&var_name)
            }
            _ => {
                panic!("Err in lambda~~")
            }
        };

        self.ctx = con_next;

        //consume Dot
        self.next_token();

        let body = self.term();

        self.ctx = prev_ctx;
        if let Some(body) = body {
            Some(Term::TmAbs(name, Box::new(body)))
        } else {
            None
        }
    }

    fn term(&mut self) -> Option<Term> {
        match self.tok0 {
            Some(Token::Lambda) => {
                self.lambda()
            }
            _ => {
                self.application()
            }
        }
    }

    pub fn parse_if_then_expr(&mut self) -> Option<Term> {
        self.next_token(); //If
        //println!("{:?}", self.tok0);

        let cond = self.atom();
        self.next_token(); //then
        let then_expr = self.atom();
        //println!("{:?}", self.tok0);
        if self.tok0 == Some(Token::Else) {
            self.next_token(); //else
            let else_expr = self.atom();
            return Some(Term::If(Box::new(cond.unwrap()), Box::new(then_expr.unwrap()), Option::from(Box::new(else_expr.unwrap()))));
        }
        return Some(Term::If(Box::new(cond.unwrap()), Box::new(then_expr.unwrap()), None));
    }

    fn atom(&mut self) -> Option<Term> {
        //println!("{:?}", self.tok0);
        match self.tok0.clone() {
            Some(Token::LParen) => {
                self.next_token();
                let term = self.term();
                self.next_token();
                term
            }
            Some(Token::Number(num)) => {
                self.next_token();
                Some(Term::Number(num))
            }
            Some(Token::Succ) => {
                self.next_token();
                let tmp = self.atom().unwrap();
                Some(Term::Succ(Box::from(tmp)))
            }
            Some(Token::Pred) => {
                self.next_token();
                Some(Term::Pred(Box::from(self.atom().unwrap())))
            }
            Some(Token::Suc) => {
                self.next_token();
                Some(suc_term())
            }
            Some(Token::Prd) => {
                self.next_token();
                Some(prd_term())
            }
            Some(Token::Plus) => {
                self.next_token();
                Some(plus_term())
            }
            Some(Token::Mult) => {
                self.next_token();
                Some(mult_term())
            }
            Some(Token::Sub) => {
                self.next_token();
                Some(sub_term())
            }
            Some(Token::True) => {
                self.next_token();
                Some(Term::True)
            }
            Some(Token::False) => {
                self.next_token();
                Some(Term::False)
            }
            Some(Token::Zero) => {
                self.next_token();
                Some(Term::Zero)
            }
            Some(Token::IsZero) => {
                self.next_token();
                let tmp = self.atom().unwrap();
                Some(Term::IsZero(Box::from(tmp)))
            }
            Some(Token::If) => {
                self.parse_if_then_expr()
            }
            Some(Token::Var(var_name)) => {
                self.next_token();
                match self.ctx.name_to_index(var_name.as_ref()) {
                    Some(idx) => {
                        Some(Term::TmVar(idx, self.ctx.len()))
                    }
                    _ => panic!("Unbound variable~~")
                }
            }
            _ => None
        }
    }

    fn application(&mut self) -> Option<Term> {
        if let Some(mut lhs) = self.atom() {
            while let Some(rhs) = self.atom() {
                lhs = Term::TmApp(Box::new(lhs), Box::new(rhs));
            }
            return Some(lhs);
        }
        None
    }

    pub fn parse(&mut self) -> Option<Term> {
        self.term()
    }

    fn next_token(&mut self) -> Option<Token> {
        let tmp_token = self.tok0.clone();
        self.tok0 = self.token.next();

        tmp_token
    }
}