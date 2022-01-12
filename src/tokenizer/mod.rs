#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialSymbol {
    LBrace,                   // (
    RBrace,                   // )
    Comma,                    // ,
    Dot,                      // .
    Dots,                     // :
    Eq,                       // =
    LThan,                    // <
    GThan,                    // >
    Minus                     // -
}

#[derive(Clone, Debug)]
pub enum Constant {
    Literal(String),
    Number(u32)
}

#[derive(Clone, Debug)]
pub enum Token {
    Constant(String),
    Number(u32),
    Variable(String),
    SpecialSymbol(SpecialSymbol),
    Whitespace(String),
}


#[derive(Debug, PartialEq, Eq)]
pub enum TokenizerError {
    BaseError{ position: usize, msg: String }
}

pub struct Tokenizer {
    input: Vec<char>,
    pos: usize
}

impl<'a> Tokenizer {
    pub fn from_str(input_str: &str) -> Self {
        Tokenizer { input: input_str.chars().collect(), pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();
        while self.input.len() > self.pos {
            if let Some(ch) = self.seek_char() {
                let token: Token;
                if ch.is_whitespace() {
                    self.next_char();
                    continue;
                }
                else if ch.is_uppercase() {
                    token = self.parse_variable()?;
                }
                else if ch.is_lowercase() || ch.is_digit(10) {
                    token = self.parse_constant()?;
                }
                else {
                    token = self.parse_special_symbol()?; 
                }
                tokens.push(token);
                
            }
        }
        Ok(tokens)
    }

    fn parse_variable(&mut self) -> Result<Token, TokenizerError> {
        let uppercase = match self.seek_char() {
            Some(ch) => ch.is_uppercase(),
            None => return Err(TokenizerError::new_base_error(self.pos, "No character"))
        };
        if !uppercase {
            return Err(TokenizerError::new_base_error(self.pos, "Char must be uppercase"));
        }

        let mut variable = String::new();

        while let Some(ch) = self.next_char() {
            if Constant::starts_with(ch) {
                variable.push(ch);
            }
            else {
                self.previous_char();
                break;
            }
        }
        Ok(Token::Variable(variable))
    }

    fn parse_constant(&mut self) -> Result<Token, TokenizerError> {
        let valid_prefix_constant = match self.seek_char() {
            Some(ch) => Constant::starts_with(ch),
            None => return Err(TokenizerError::new_base_error(self.pos, "No character"))
        };
        if !valid_prefix_constant {
            return Err(TokenizerError::new_base_error(self.pos, "Char is not a valid prefix"));
        }

        match self.parse_integer() {
            Ok(token) => return Ok(token),
            Err(_) => ()
        };

        let mut literal = String::new();
        
        while let Some(ch) = self.next_char() {
            if Constant::starts_with(ch) {
                literal.push(ch);
            }
            else {
                self.previous_char();
                break;
            }
        }
        Ok(Token::Constant(literal))
    }

    fn parse_integer(&mut self) -> Result<Token, TokenizerError> {
        let is_digit = match self.seek_char() {
            Some(ch) => ch.is_digit(10),
            None => return Err(TokenizerError::new_base_error(self.pos, "No character"))
        };
        if !is_digit {
            return Err(TokenizerError::new_base_error(self.pos, "Char is not digit"));
        }

        let mut number = 0; 

        while let Some(ch) = self.next_char() {
            if let Some(v) = ch.to_digit(10) {
                number += number*10 + v;
            }
            else {
                self.previous_char();
                break;
            }
        }
        Ok(Token::Number(number))
    }

    fn parse_special_symbol(&mut self) -> Result<Token, TokenizerError> {

        let is_special_symbol = match self.seek_char() {
            Some(ch) => SpecialSymbol::starts_with(ch),
            None => return Err(TokenizerError::new_base_error(self.pos, "No character"))
        };

        if !is_special_symbol {
            return Err(TokenizerError::new_base_error(self.pos, "Unrecognised character"));
        }
        let token: Token;
        if let Some(ch) = self.next_char() {
            token = match ch {
                '(' => Token::SpecialSymbol(SpecialSymbol::LBrace),
                ')' => Token::SpecialSymbol(SpecialSymbol::RBrace),
                ',' => Token::SpecialSymbol(SpecialSymbol::Comma),
                '.' => Token::SpecialSymbol(SpecialSymbol::Dot),
                ':' => Token::SpecialSymbol(SpecialSymbol::Dots),
                '=' => Token::SpecialSymbol(SpecialSymbol::Eq),
                '<' => Token::SpecialSymbol(SpecialSymbol::LThan),
                '>' => Token::SpecialSymbol(SpecialSymbol::GThan),
                '-' => Token::SpecialSymbol(SpecialSymbol::Minus),
                _ => return Err(TokenizerError::new_base_error(self.pos, "Character is not a special symbol"))
            };
            return Ok(token);
        }
        return Err(TokenizerError::new_base_error(self.pos, "Character is not a special symbol"));
    }

    fn seek_char(&self) -> Option<char> {
        if self.input.len() <= self.pos {
            return None;
        }
        Some(self.input[self.pos])
    }

    fn next_char(&mut self) -> Option<char> {
        let ch: char;
        if self.input.len() <= self.pos {
            return None;
        } else {
            ch = self.input[self.pos];
            self.pos += 1;
        };
        Some(ch)
    }

    fn previous_char(&mut self) -> Option<char> {
        let ch: char;
        if self.pos <= 0 {
            return None;
        } else {
            self.pos -= 1;
            ch = self.input[self.pos];           
        };
        Some(ch)
    }
}

pub trait StartsWith {
    fn starts_with(ch: char) -> bool;
}

impl StartsWith for SpecialSymbol {
    fn starts_with(ch: char) -> bool {
        let options = "(),.:=<>-";
        options.contains(ch)
    }
}

impl StartsWith for Constant {
    fn starts_with(ch: char) -> bool {
        ch.is_digit(10) || ch.is_alphabetic() || ch == '_'
    }
}

impl TokenizerError {
    pub fn new_base_error(position: usize, msg: &str) -> Self {
        TokenizerError::BaseError{position, msg: String::from(msg)}
    }
}