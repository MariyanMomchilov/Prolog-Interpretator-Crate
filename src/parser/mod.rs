#[cfg(test)]
mod tests;

use std::boxed;

use super::tokenizer::*;

pub trait Clause {
    fn evaluate(&self) {}
    fn unify(&self) {}
}

pub struct Variable(String);

pub struct Constant(String);

pub struct Fact {
    name: String,
    args: Vec<Box<dyn Clause>>,
}

pub struct Rule {
    head: Fact,
    body: Vec<Box<dyn Clause>>,
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
        todo!()
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

    fn parse_fact_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let constant = self.get_constant()?;
    }

    // fn parse_clause(&mut self) -> Result<Box<dyn Clause>, ParserError> {
    //     if let Ok(boxed_var) = self.parse_variable() {
    //         return Ok(boxed_var);
    //     }

    //     let token = self.next_token();
    //     let constant_result = self.get_constant(&token);
    //     if let Ok(c) = constant_result {

    //         let token = self.next_token();

    //         if let Ok(()) = self.is_symbol(&token, SpecialSymbol::LBrace) {

    //             let mut args =  Vec::<Box<dyn Clause>>::new();
    //             let mut token = self.next_token();

    //             while let Err(_) = self.is_symbol(&token, SpecialSymbol::RBrace) {
    //                 self.advance_pos(-1);
    //                 let arg = self.parse_clause()?;
    //                 args.push(arg);
    //                 token = self.next_token();
    //                 if let Ok(()) = self.is_symbol(&token, SpecialSymbol::Comma) {
    //                     token = self.next_token();
    //                 }
    //             }

    //             let fact = Fact{ name: c, args };

    //             let tokenf = self.next_token();
    //             let tokens = self.next_token();

    //             if let Ok(()) = self.is_implication(&tokenf, &tokens) {
    //                 let mut body = Vec::<Box<dyn Clause>>::new();
    //                 let mut clause = self.parse_clause()?;
    //                 body.push(clause);
    //                 let mut token = self.next_token();
    //                 while let Err(_) = self.is_symbol(&token, SpecialSymbol::Dot) {
    //                     self.advance_pos(-1);
    //                     clause = self.parse_clause()?;
    //                     body.push(clause);
    //                     token = self.next_token();
    //                     if let Ok(()) = self.is_symbol(&token, SpecialSymbol::Comma) {
    //                         token = self.next_token();
    //                     }
    //                 }
    //                 return Ok(Box::new(Rule{head: fact, body}));
    //             }
    //             else {
    //                 self.advance_pos(-1);
    //                 match self.is_symbol(&tokenf, SpecialSymbol::Dot) {
    //                     Ok(_) => return Ok(Box::new(fact)),
    //                     Err(_) => match self.is_symbol(&tokenf, SpecialSymbol::RBrace) {
    //                         Ok(_) => return Ok(Box::new(fact)),
    //                         Err(e) => return Err(e)
    //                     }
    //                 };
    //             }
    //         }
    //         else {
    //             self.advance_pos(-1);
    //             return Ok(Box::new(Constant(c)));
    //         }
    //     }
    //     Err(constant_result.unwrap_err())
    // }

    fn parse_variable(&mut self) -> Result<Box<dyn Clause>, ParserError> {
        let token = self.next_token();
        let r = self.get_variable(&token);
        if let Ok(var) = r {
            return Ok(Box::new(Variable(var)));
        }
        self.advance_pos(-1);
        Err(r.unwrap_err())
    }

    fn is_implication(
        &self,
        tokenf: &Option<Token>,
        tokens: &Option<Token>,
    ) -> Result<(), ParserError> {
        self.is_symbol(tokenf, SpecialSymbol::Dots)?;
        self.is_symbol(tokens, SpecialSymbol::Minus)?;
        Ok(())
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
        &self,
        token: &Option<Token>,
        special_symbol: SpecialSymbol,
    ) -> Result<(), ParserError> {
        match token {
            None => Err(ParserError::BaseError {
                msg: String::from("No token"),
            }),
            Some(token) => {
                if let Token::SpecialSymbol(s) = token {
                    if *s == special_symbol {
                        return Ok(());
                    }
                }
                return Err(ParserError::expected_error(
                    self.pos,
                    &Token::SpecialSymbol(special_symbol),
                    token,
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
                t => Err(ParserError::expected_error(
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

impl Clause for Variable {}

impl Clause for Constant {}

impl Clause for Fact {}

impl Clause for Rule {}
