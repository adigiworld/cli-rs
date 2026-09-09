use std::fmt;

use crate::parsemath::{
    token::{OperPrec, Token},
    tokenizer::Tokenizer,
};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

#[derive(Clone, Debug)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Devide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Number(f64),
}

#[derive(Debug)]
pub enum PareseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, PareseError> {
        let mut lexer = Tokenizer::new(expr);
        let current_token = match lexer.next() {
            Some(token) => token,
            None => return Err(PareseError::InvalidOperator("Invalid character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token,
        })
    }
    pub fn parse(&mut self) -> Result<Node, PareseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(err) => Err(err),
        }
    }
    fn get_next_token(&mut self) -> Result<(), PareseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(PareseError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())
    }
    fn check_paren(&mut self, expected: Token) -> Result<(), PareseError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(PareseError::InvalidOperator(format!(
                "Expected: {:?}, got: {:?}",
                expected, self.current_token
            )))
        }
    }
    fn parse_number(&mut self) -> Result<Node, PareseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.current_token == Token::RightParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(PareseError::UnableToParse("Unable to parse".to_string())),
        }
    }
    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, PareseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, PareseError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Devide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(PareseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

impl fmt::Display for PareseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            Self::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}

impl std::convert::From<std::boxed::Box<dyn std::error::Error>> for PareseError {
    fn from(_evalue: std::boxed::Box<dyn std::error::Error>) -> Self {
        return PareseError::UnableToParse("Unable to parse".into());
    }
}
