use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum TokenValue {
    LParen,
    RParen,
    Symbol(String),
    Number(f32),
    String(String),
    Quote,
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Token {
    pub value: TokenValue,
    pub location: Location,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token[{},{}]({:?})",
            self.location.row, self.location.col, self.value
        )
    }
}

pub struct Tokenizer {
    input: String,
    pub location: Location,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum LexerError {
    TryFailed,
    MismatchedQuote,
}

fn is_symbol(c: char) -> bool {
    c.is_alphanumeric() || "+-*/%!^&|~<=>".contains(c)
}

impl Tokenizer {
    pub fn new(input: String) -> Tokenizer {
        Tokenizer {
            input,
            location: Location { row: 0, col: 0 },
            tokens: Vec::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().next()
    }

    fn next(&mut self) -> Option<char> {
        let mut chars = self.input.chars();
        match chars.next() {
            Some(c) => {
                if c == '\n' {
                    self.location.row += 1;
                    self.location.col = 0;
                } else {
                    self.location.col += 1;
                }
                self.input = chars.collect();
                Some(c)
            }
            None => None,
        }
    }

    fn trim(&mut self) {
        while let Some(c) = self.peek()
            && c.is_whitespace()
        {
            self.next();
        }
    }

    fn push_token(&mut self, tok: TokenValue) {
        self.tokens.push(Token {
            location: self.location,
            value: tok,
        });
    }

    pub fn try_parse_one(&mut self) -> Result<(), LexerError> {
        self.trim();

        if let Some(c) = self.peek() {
            match c {
                '(' => {
                    self.push_token(TokenValue::LParen);
                    self.next();
                }
                ')' => {
                    self.push_token(TokenValue::RParen);
                    self.next();
                }
                '\'' => {
                    self.push_token(TokenValue::Quote);
                    self.next();
                }
                '"' => {
                    self.next();
                    let mut s = String::new();
                    loop {
                        if let Some(c) = self.next() {
                            if c == '"' {
                                break;
                            } else {
                                s.push(c);
                            }
                        } else {
                            return Err(LexerError::MismatchedQuote);
                        }
                    }
                    self.push_token(TokenValue::String(s));
                }
                '0'..'9' => {
                    let mut digits: f32 = 0.0;
                    let mut floating: f32 = 1.0;

                    while let Some(c) = self.peek()
                        && c.is_digit(10)
                    {
                        self.next();
                        digits = 10.0 * digits + c.to_digit(10).unwrap() as f32;
                    }

                    if let Some('.') = self.peek() {
                        self.next();
                        while let Some(c) = self.peek()
                            && c.is_digit(10)
                        {
                            self.next();
                            digits = 10.0 * digits + c.to_digit(10).unwrap() as f32;
                            floating *= 10.0;
                        }
                    }

                    self.push_token(TokenValue::Number(digits / floating));
                }
                c if is_symbol(c) => {
                    let mut sym = String::new();
                    while let Some(c) = self.peek()
                        && is_symbol(c)
                    {
                        self.next();
                        sym.push(c);
                    }
                    self.push_token(TokenValue::Symbol(sym));
                }
                _ => return Err(LexerError::TryFailed),
            }
        }

        Ok(())
    }

    pub fn try_parse_all(&mut self) -> Result<(), LexerError> {
        loop {
            if let None = self.peek() {
                return Ok(());
            }

            match self.try_parse_one() {
                Ok(()) => {}
                Err(LexerError::TryFailed) => {
                    if self.peek().is_none() {
                        return Ok(());
                    } else {
                        return Err(LexerError::TryFailed);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}
