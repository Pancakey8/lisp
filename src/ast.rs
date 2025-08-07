use std::fmt::Debug;

use crate::token::{Location, Token, TokenValue};

#[derive(Clone, Debug)]
pub enum ASTNodeValue {
    List(Vec<ASTNode>),
    Ident(String),
    Number(f32),
    String(String),
    Quote(Box<ASTNode>),
}

#[derive(Clone)]
pub struct ASTNode {
    pub location: Location,
    pub value: ASTNodeValue,
}

impl Debug for ASTNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node[{},{}]({:?})",
            self.location.row, self.location.col, self.value
        )
    }
}

pub struct ASTParser {
    pub input: Vec<Token>,
    pub roots: Vec<ASTNode>,
}

#[derive(Debug)]
pub enum ASTParserError {
    TryFailed,
    MismatchedParenthesis,
}

impl ASTParser {
    pub fn new(input: Vec<Token>) -> ASTParser {
        ASTParser {
            input,
            roots: Vec::new(),
        }
    }

    pub fn peek(&self) -> Option<Token> {
        self.input.iter().next().map(|t| t.clone())
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.input.is_empty() {
            None
        } else {
            Some(self.input.remove(0))
        }
    }

    pub fn push_node(&mut self, node: ASTNodeValue, location: Location) {
        self.roots.push(ASTNode {
            location,
            value: node,
        });
    }

    pub fn try_parse_one(&mut self) -> Result<(), ASTParserError> {
        if let Some(t) = self.peek() {
            match t.value {
                TokenValue::Quote => {
                    self.next();
                    let mut p = ASTParser::new(std::mem::take(&mut self.input));
                    match p.try_parse_one() {
                        Err(e) => return Err(e),
                        Ok(()) => {
                            self.push_node(
                                ASTNodeValue::Quote(Box::new(p.roots.remove(0))),
                                t.location,
                            );
                            self.input = p.input;
                        }
                    }
                }
                TokenValue::Number(n) => {
                    self.next();
                    self.push_node(ASTNodeValue::Number(n), t.location);
                }
                TokenValue::Symbol(s) => {
                    self.next();
                    self.push_node(ASTNodeValue::Ident(s), t.location);
                }
                TokenValue::String(s) => {
                    self.next();
                    self.push_node(ASTNodeValue::String(s), t.location);
                }
                TokenValue::LParen => {
                    self.next();

                    let mut args = Vec::new();
                    loop {
                        if let Some(t) = self.peek() {
                            match t.value {
                                TokenValue::RParen => {
                                    self.next();
                                    break;
                                }
                                _ => {
                                    let mut local_parser =
                                        ASTParser::new(std::mem::take(&mut self.input));
                                    if let Err(e) = local_parser.try_parse_one() {
                                        return Err(e);
                                    }
                                    self.input = local_parser.input;
                                    args.push(local_parser.roots.remove(0));
                                }
                            }
                        } else {
                            return Err(ASTParserError::MismatchedParenthesis);
                        }
                    }

                    self.push_node(ASTNodeValue::List(args), t.location);
                }
                _ => return Err(ASTParserError::TryFailed),
            }
        }

        Ok(())
    }

    pub fn try_parse_all(&mut self) -> Result<(), (Location, ASTParserError)> {
        loop {
            match self.peek() {
                None => return Ok(()),
                Some(tok) => match self.try_parse_one() {
                    Ok(()) => {}
                    Err(ASTParserError::TryFailed) => {
                        if self.peek().is_none() {
                            return Ok(());
                        } else {
                            return Err((tok.location, ASTParserError::TryFailed));
                        }
                    }
                    Err(e) => return Err((tok.location, e)),
                },
            }
        }
    }
}
