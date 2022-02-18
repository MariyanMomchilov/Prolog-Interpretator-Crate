#[cfg(test)]
mod tests;

use std::{fmt::{Formatter, Debug, Error}};

use super::tokenizer::*;
use super::interpretator::*;

pub struct Variable(pub String);

pub struct Constant(pub String);

pub struct Fact {
    pub name: String,
    pub args: Vec<Box<dyn Clause>>,
}

pub struct Rule {
    pub head: Box<dyn Clause>,
    pub body: Vec<Box<dyn Clause>>,
}

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ParserError {
    BaseError { msg: String },
}

impl ParserError {
    pub fn expected_error(pos: usize, token_e: &Token, token_r: &Token) -> ParserError {
        Self::BaseError {
            msg: format!(
                "Expected {:?} at pos: {}, found {:?}",
                token_e, pos, token_r
            ),
        }
    }
}

impl Parser {
    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Clause>>, ParserError> {
        let mut clauses = Vec::new();

        while self.has_tokens() {
            let clause = self.parse_clause()?;
            clauses.push(clause);
        }

        Ok(clauses)
    }

    fn parse_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let variable_clause = self.parse_variable_clause();
        if let Ok(v) = variable_clause {
            return Ok(v);
        }
        if let Ok(r) = self.parse_rule_clause() {
            return Ok(r);
        }
        let fact = self.parse_fact_clause()?;
        Ok(fact)
    }

    fn parse_variable_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let var = self.parse_variable()?;
        if let Some(token) = self.next_token() {
            return match token {
                Token::SpecialSymbol(SpecialSymbol::Dot) => Ok(var),
                t => {
                    self.advance_pos(-1);
                    return Err(ParserError::expected_error(
                        self.pos,
                        &Token::SpecialSymbol(SpecialSymbol::Dot),
                        &t,
                    ));
                },
            };
        }
        return Err(ParserError::BaseError {
            msg: String::from("Expected Token, found None"),
        });
    }

    fn parse_rule_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let current_pos = self.pos;
        let name = self.get_constant()?;
        let head = self.parse_functor(name)?;
        if let Err(e) = self.is_symbol(SpecialSymbol::Dots) {
            self.pos = current_pos;
            return Err(e);
        }

        self.is_symbol(SpecialSymbol::Minus)?;
        
        let mut body = Vec::new();
        while let Err(_) = self.is_symbol(SpecialSymbol::Dot) {
            let name = self.get_constant()?;
            let body_functor = self.parse_functor(name)?;
            body.push(body_functor);
            self.is_symbol(SpecialSymbol::Comma);
        }
        Ok(Box::new(Rule{head, body}))
    }

    fn parse_fact_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let constant = self.get_constant()?;
        let functor = self.parse_functor(constant)?;
        self.is_symbol(SpecialSymbol::Dot)?;
        Ok(functor)
    }

    fn parse_functor(&mut self, name: String) -> Result<Box<dyn Clause>, ParserError> {
        self.is_symbol(SpecialSymbol::LBrace)?;
        let mut args = Vec::<Box<dyn Clause>>::new();
        while let Err(_) = self.is_symbol(SpecialSymbol::RBrace) {
            let arg = self.parse_arg()?;
            args.push(arg);
            self.is_symbol(SpecialSymbol::Comma);
        }
        Ok(Box::new(Fact{ name, args }))
    }

    fn parse_arg(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let var = self.parse_variable();
        if let Ok(boxed_var) = var {
            return Ok(boxed_var);
        }
        let constant = self.get_constant()?;
        if let Ok(functor) = self.parse_functor(constant.clone()) {
            return Ok(functor);
        }
        Ok(Box::new(Constant(constant)))
    }

    fn parse_variable(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let token = self.next_token();
        let r = self.get_variable(&token);
        if let Ok(var) = r {
            return Ok(Box::new(Variable(var)));
        }
        self.advance_pos(-1);
        Err(r.unwrap_err())
    }

    fn get_constant(&mut self) -> Result<String, ParserError> {
        let token = self.next_token();
        match token {
            None => Err(ParserError::BaseError {
                msg: String::from("No token"),
            }),
            Some(token) => match token.clone() {
                Token::Constant(s) => Ok(s),
                t => {
                    self.advance_pos(-1);
                    return Err(ParserError::expected_error(
                        self.pos,
                        &Token::Constant(String::from("Some")),
                        &t,
                    ));
                },
            },
        }
    }

    fn is_symbol(
        &mut self,
        special_symbol: SpecialSymbol,
    ) -> Result<(), ParserError> {
        let token = self.next_token();
        match token {
            None => Err(ParserError::BaseError {
                msg: String::from("No token"),
            }),
            Some(token) => {
                if let Token::SpecialSymbol(s) = token {
                    if s == special_symbol {
                        return Ok(());
                    }
                }
                self.advance_pos(-1);
                return Err(ParserError::expected_error(
                    self.pos,
                    &Token::SpecialSymbol(special_symbol),
                    &token,
                ));
            }
        }
    }

    fn get_variable(&self, token: &Option<Token>) -> Result<String, ParserError> {
        match token {
            None => Err(ParserError::BaseError {
                msg: String::from("No token"),
            }),
            Some(token) => match token.clone() {
                Token::Variable(s) => Ok(s),
                _ => Err(ParserError::expected_error(
                    self.pos,
                    &Token::Variable(String::from("Some")),
                    token,
                )),
            },
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).map(|token_ref| token_ref.clone());
        self.advance_pos(1);
        t
    }

    fn has_tokens(&self) -> bool {
        self.pos < self.tokens.len()
    }

    fn advance_pos(&mut self, p: i32) {
        self.pos = (self.pos as i32 + p) as usize;
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("Variable({})", &self.0));
        Ok(())
    }
}

impl Debug for Constant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("Constant({})", &self.0));
        Ok(())
    }
}

impl Debug for Fact {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("Fact({}, {:?})", &self.name, &self.args));
        Ok(())
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("Rule({:?}, {:?})", &self.head, &self.body));
        Ok(())
    }
}
