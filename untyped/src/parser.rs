use std::fmt::Debug;
use crate::core::{Term, Context};

#[derive(Clone,Debug, PartialEq)]
pub enum Token {
    Lambda,
    Var(String),

    Dot,
    LParen,
    RParen,
}

/*
fn get_keywords() -> HashMap<String, Token> {
    let mut result = HashMap::new();

    result.insert(String::from("lambda"), Token::Lambda);
    result
}*/

pub struct Lexer<T: Iterator<Item = char> + Debug> {
    chars: T,
    chr0: Option<char>,
    chr1: Option<char>,
    //keywords: HashMap<String, Token>,

    pub result: Vec<Token>,

}


impl<T> Lexer<T>
    where
        T: Iterator<Item = char> + Debug {
    pub fn new(input: T) -> Self {
        let mut lex = Lexer {
            chars: input,
            chr0: None,
            chr1: None,
            //keywords: get_keywords(),
            result: vec![]
        };
        lex.next_char();
        lex.next_char();
        lex
    }

    pub fn lex_input(&mut self) {
        //(lambda x. x) (lambda x. x x);
        loop {
            if self.chr0 == None {
                break;
            }

            match self.chr0.unwrap() {
                'a'..='z' | 'A'..='Z' => {
                    let mut result = Vec::new();
                    loop {
                        if self.chr0 == None || !self.chr0.unwrap().is_ascii_alphabetic() {
                            break;
                        }

                        result.push(self.chr0.unwrap().to_string());
                        self.next_char();
                    }
                    let name = result.join("");
                    self.result.push(Token::Var(name));
                }
                'λ' => {
                    self.result.push(Token::Lambda);
                    self.next_char();
                }
                '(' => {
                    self.result.push(Token::LParen);
                    self.next_char();
                }
                ')' => {
                    self.result.push(Token::RParen);
                    self.next_char();
                }
                '.' => {
                    self.result.push(Token::Dot);
                    self.next_char();
                }
                ' ' => {
                    self.next_char();
                },
                _ => {}
            }
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let tmp_char = self.chr0;
        self.chr0 = self.chr1;
        self.chr1 = self.chars.next();

        tmp_char
    }
}


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

    fn atom(&mut self) -> Option<Term> {
        match self.tok0.clone() {
            Some(Token::LParen) => {
                self.next_token();
                let term = self.term();
                self.next_token();
                term
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