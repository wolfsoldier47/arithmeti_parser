use std::{fmt::{self, write}, result};

use super::{token, tokenizer::Tokenizer,ast};

use super::ast::Node;



// Custom Error handling
pub enum ParseError {
  UnableToParse(String),
  InvalidOperator(String),
}



pub struct Parser<'a> {
  tokenizer: Tokenizer<'a>,
  current_token: token::Token,
}


impl<'a> Parser<'a>{
  fn new(expr: &'a str) -> Result<Self,ParseError> {

    let mut lexer = Tokenizer::new(expr);
    let cur_token = match lexer.next() {
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    };
    Ok(Parser { tokenizer: lexer, current_token: cur_token })
  }

  pub fn parse(&mut self) -> Result<ast::Node, ParseError>{
    let ast = self.generate_ast(token::OperPrec::DefaultZero);
    match ast {
      Ok(ast)=> Ok(ast),
      Err(e) => Err(e),
    }
  }
}

//private methods
impl<'a> Parser<'a>{

  //updating the current struct current_token field with next Token thats why no return
  fn get_next_token(&mut self) -> Result<(),ParseError> {
    let next_token = match self.tokenizer.next(){
      Some(token) => token,
      None => return Err(ParseError::InvalidOperator("Invalid Character".into()))
    };
    self.current_token = next_token;
    Ok(())
  }
  fn check_paren(&mut self,expected: token::Token) -> Result<(),ParseError>{
    if expected == self.current_token {
      self.get_next_token()?;
      Ok(())
    } else {
      Err(ParseError::InvalidOperator(format!(
        "Expected {:?}, got {:?}",
        expected, self.current_token
      )))
    }
  }
  fn parse_number(&mut self)-> Result<Node, ParseError>{
    let token = self.current_token.clone();
    match token {
      token::Token::Subtract => {
        self.get_next_token()?;
        let expr = self.generate_ast(token::OperPrec::Negative)?;
        Ok(Node::Negative(Box::new(expr)))
      }
      token::Token::Num(i) =>{
        self.get_next_token()?;
        Ok(Node::Number(i))
      }
      token::Token::LeftParen => {
        self.get_next_token()?;
        let expr = self.generate_ast(token::OperPrec::DefaultZero)?;
        self.check_paren(token::Token::RightParen)?;
        if self.current_token == token::Token::LeftParen {
          let right = self.generate_ast(token::OperPrec::MulDiv)?;
          return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
        }
        Ok(expr)
      }
      _ => Err(ParseError::UnableToParse("Unable to parse".to_string()))
    }
  }

  fn generate_ast(&mut self,oper_prec: token::OperPrec) -> Result<Node,ParseError> {
    let mut left_expr = self.parse_number()?;
    while oper_prec < self.current_token.get_oper_prec() {
      if self.current_token == token::Token::EOF {
        break;
      }
      let right_expr = self.convert_token_to_node(left_expr.clone())?;
      left_expr = right_expr;
    }
    Ok(left_expr)
  }

  fn convert_token_to_node(&mut self, left_expr:Node) -> Result<Node, ParseError>{
    match self.current_token {
      token::Token::Add =>{
        self.get_next_token()?;
        //get right side 
        let right_expr = self.generate_ast(token::OperPrec::AddSub)?;
        Ok(Node::Add(Box::new(left_expr),Box::new(right_expr)))
      }
      token::Token::Subtract => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(token::OperPrec::AddSub)?;
        Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
      }
      token::Token::Multiply => {
        self.get_next_token()?;
        let right_expr = self.generate_ast(token::OperPrec::MulDiv)?;
        Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
      }
      token::Token::Divide =>{
        self.get_next_token()?;
        let right_expr = self.generate_ast(token::OperPrec::MulDiv)?;
        Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
      }
      token::Token::Caret => {
        self.get_next_token()?;
        //get right-side
        let right_expr = self.generate_ast(token::OperPrec::Power)?;
        Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
      }
      _ => Err(ParseError::InvalidOperator(format!(
        "Please enter valid operator {:?}",
        self.current_token
      ))),
    }
  }
}


impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      self::ParseError::UnableToParse(e) => write!(f,"Error in evaluation {}",e),
            self::ParseError::InvalidOperator(e) => write!(f,"Error in evaluation {}",e),
    }
  }
}