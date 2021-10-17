use crate::nodes::*;
use crate::tokens::*;


pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tk: Vec<Token>) -> Parser {
        Parser {tokens: tk}
    }
    pub fn parse(&self) {
        let mut tokens_iter = self.tokens.iter();

        self.expr(tokens_iter);
    }

    pub fn expr(&mut self, tokens_iter: Iterator) -> 

}