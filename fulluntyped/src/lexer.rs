use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone,Debug, Eq, Hash, PartialEq)]
pub enum Token {
    Lambda,
    Var(String),
    Number(u64),
    True,
    False,
    Zero,
    IsZero,
    Succ,
    Pred,
    If,
    Then,
    Else,

    Sub,
    Mult,
    Plus,
    Dot,      //.
    LParen,   //(
    RParen,   //)
}


fn get_keywords() -> HashMap<String, Token> {
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
            keywords: get_keywords(),
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

                        result.push(self.chr0);
                        self.next_char();
                    }
                    let name = result.iter().map(|&x| x.unwrap().to_string()).collect::<Vec<String>>().join("");
                    if let Some(out) = self.keywords.get(&name) {
                        self.result.push(out.to_owned());
                    } else {
                        self.result.push(Token::Var(name));
                    }
                }
                '0'..='9' => {
                    let mut result = String::new();
                    loop {
                        if self.chr0 == None || !self.chr0.unwrap().is_digit(10) {
                            break;
                        }

                        result = result + self.chr0.unwrap().to_string().as_str();
                        self.next_char();
                    }
                    self.result.push(Token::Number(result.parse::<u64>().unwrap()));
                }
                '*' => {
                    self.result.push(Token::Mult);
                    self.next_char();
                }
                '+' => {
                    self.result.push(Token::Plus);
                    self.next_char();
                }
                '-' => {
                    self.result.push(Token::Sub);
                    self.next_char();
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
