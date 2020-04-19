use std::collections::HashMap;
use std::fmt::Debug;
use crate::tyarith::Term;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    True,
    False,
    Zero,
    IsZero,
    Succ,
    Pred,
    If,
    Then,
    Else,

    LParen,
    RParen,
}

pub fn get_keywords() -> HashMap<String, Token> {
    let mut result = HashMap::new();

    result.insert(String::from("if"), Token::If);
    result.insert(String::from("else"), Token::Else);
    result.insert(String::from("zero"), Token::Zero);
    result.insert(String::from("iszero"), Token::IsZero);
    result.insert(String::from("true"), Token::True);
    result.insert(String::from("false"), Token::False);
    result.insert(String::from("succ"), Token::Succ);
    result.insert(String::from("pred"), Token::Pred);
    result.insert(String::from("then"), Token::Then);
    result
}


pub struct Lexer<T: Iterator<Item = char> + Debug> {
    chars: T,
    chr0: Option<char>,
    chr1: Option<char>,
    keywords: HashMap<String, Token>,

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
            result: vec![],
            keywords: get_keywords()
        };
        lex.next_char();
        lex.next_char();
        lex
    }
    //is_ascii_alphabetic

    pub fn lex_input(&mut self) {

        loop {
            if self.chr0 == None {
                break;
            }

            match self.chr0.unwrap() {
                'a'..='z' => {
                    let mut result = Vec::new();
                    loop {
                        if self.chr0 == None || !self.chr0.unwrap().is_ascii_alphabetic() {
                            break;
                        }

                        result.push(self.chr0);
                        self.next_char();
                    }
                    let key_name = result.iter().map(|&x| x.unwrap().to_string()).collect::<Vec<String>>().join("");
                    let out = self.keywords.get(&key_name).unwrap().to_owned();
                    self.result.push(out);
                }
                '(' => {
                    //self.result.push(Token::LParen);
                    self.next_char();
                }
                ')' => {
                    //self.result.push(Token::RParen);
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
        let result = self.chr0;
        self.chr0 = self.chr1;
        self.chr1 = self.chars.next();
        result
    }
}

#[derive(Clone, PartialEq)]
pub struct Parser<T: Iterator<Item = Token> + Debug + Clone> {
    token: T,
    tok0: Option<Token>,
    pub result: Option<Term>,
}

impl<T> Parser<T>
    where
        T: Iterator<Item = Token> + Debug + Clone{
    pub fn new(input: T) -> Self {

        let mut parser = Parser {
            token: input,
            tok0: None,
            result: None
        };
        parser.next_token();
        parser
    }

    pub fn expr(&mut self) -> Option<Term> {
        match self.tok0 {
            Some(Token::False) => {
                self.next_token();
                Some(Term::False)
            },
            Some(Token::True) => {
                self.next_token();
                Some(Term::True)
            },
            Some(Token::Zero) => {
                self.next_token();
                Some(Term::Zero)
            },
            Some(Token::Succ) => {
                self.next_token();
                let tmp = self.expr().unwrap();
                Some(Term::Succ(Box::from(tmp)))
            }
            Some(Token::Pred) => {
                self.next_token();
                Some(Term::Pred(Box::from(self.expr().unwrap())))
            }
            Some(Token::If) => {
                self.parse_if_then_expr()
            }
            Some(Token::IsZero) => {
                self.next_token();
                Some(Term::IsZero(Box::new(self.expr().unwrap())))
            }
            _ => panic!("Err~~~"),
        }
    }

    pub fn parse_if_then_expr(&mut self) -> Option<Term> {
        self.next_token(); //If
        let cond = self.expr();
        self.next_token(); //then
        let then_expr = self.expr();

        if self.tok0 == Some(Token::Else) {
            self.next_token(); //else
            let else_expr = self.expr();
            return Some(Term::If(Box::new(cond.unwrap()), Box::new(then_expr.unwrap()), Option::from(Box::new(else_expr.unwrap()))));
        }
        return Some(Term::If(Box::new(cond.unwrap()), Box::new(then_expr.unwrap()), None));
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let result = self.tok0.clone();
        self.tok0 = self.token.next();

        result
    }
}

