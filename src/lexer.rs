// Generates tokens
use crate::tokens::TokenType;
use crate::tokens::Token;
use std::collections::VecDeque;

fn gen_digits() -> Vec<String> {
    let mut dig = Vec::new();
    for i in 0..10 {
        dig.push(i.to_string());
    }
    dig
}

#[derive(Debug)]
pub struct Lexer {
    pub text: String,
    pub tokens: Vec<Token>,
    number: VecDeque<String>
}

impl Lexer {
    pub fn new(txt: String) -> Lexer {

        Lexer {
            text: txt, 
            tokens: vec![Token::new(TokenType::SOF, None)],
            number: VecDeque::new()
        }
    }

    pub fn generate_tokens(&mut self) {
        let digits = gen_digits();
        let whitespaces = [" ".to_string(), "\t".to_string(), "\n".to_string(), "".to_string()];
        let _text = self.text.clone();
        let token_vector: Vec<&str> = _text.split("").collect();
        for token in &token_vector {
            if digits.contains(&token.to_string()) == true {
                self.number.push_back(token.to_string());
            } else if token.to_string() == "+".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::PLUS, None))
            } else if token.to_string() == "-".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::MINUS, None))
            } else if token.to_string() == "*".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::MULTIPLY, None))
            } else if token.to_string() == "/".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::DIVIDE, None))
            } else if token.to_string() == "(".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::LPAREN, None))
            } else if token.to_string() == ")".to_string() {
                self.check_number();
                self.tokens.push(Token::new(TokenType::RPAREN, None))
            }
            else if whitespaces.contains(&token.to_string()) == true {
                continue;
            } else {
                panic!("Illegal expression = {}", token);
            }
        }
        self.check_number();
    }

    pub fn check_number(&mut self) {
        if self.number.is_empty() == false {
            let mut s: String = String::new();
            for num in &self.number {
                s = s + num;
            }
            self.number.clear();
            self.tokens.push(Token::new(TokenType::NUMBER, Some(s.parse::<isize>().unwrap())));
        }
    }
}