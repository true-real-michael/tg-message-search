use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum SearchQuery {
    Word(String),
    And(Box<(SearchQuery, SearchQuery)>),
    Or(Box<(SearchQuery, SearchQuery)>),
}

enum ParseItem {
    And,
    Or,
    Word,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    LParen,
    RParen,
    And,
    Or,
    Word(String),
    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Result<Token> {
        while self.pos < self.input.len() {
            let current_char = self.current_char();

            if current_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if current_char.is_alphanumeric() {
                return Ok(Token::Word(self.word()));
            }

            return match current_char {
                '|' => {
                    self.pos += 1;
                    Ok(Token::Or)
                }
                '&' => {
                    self.pos += 1;
                    Ok(Token::And)
                }
                '(' => {
                    self.pos += 1;
                    Ok(Token::LParen)
                }
                ')' => {
                    self.pos += 1;
                    Ok(Token::RParen)
                }
                _ => Err(anyhow!("Unexpected character: {}", current_char)),
            };
        }
        Ok(Token::EOF)
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.pos += 1;
        }
    }

    fn word(&mut self) -> String {
        let start_pos = self.pos;
        while self.pos < self.input.len() && self.current_char().is_alphanumeric() {
            self.pos += 1;
        }
        String::from(&self.input[start_pos..self.pos])
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Result<Self> {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.current_token = parser.lexer.next_token()?;
        Ok(parser)
    }

    pub fn parse(&mut self) -> Result<SearchQuery> {
        self.expr()
    }

    fn expr(&mut self) -> Result<SearchQuery> {
        let mut result = self.term()?;

        while matches!(self.current_token, Token::Or) {
            let op = self.current_token.clone();
            self.eat(op.clone());
            let rhs = self.term()?;
            result = match op {
                Token::Or => SearchQuery::Or(Box::new((result, rhs))),
                _ => result,
            };
        }

        Ok(result)
    }

    fn term(&mut self) -> Result<SearchQuery> {
        let mut result = self.factor()?;

        while matches!(self.current_token, Token::And) {
            let op = self.current_token.clone();
            self.eat(op.clone());
            let rhs = self.factor()?;
            result = match op {
                Token::And => SearchQuery::And(Box::new((result, rhs))),
                _ => result,
            };
        }

        Ok(result)
    }

    fn factor(&mut self) -> Result<SearchQuery> {
        match self.current_token.clone() {
            Token::Word(value) => {
                self.eat(Token::Word(value.clone()));
                Ok(SearchQuery::Word(value))
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let result = self.expr()?;
                self.eat(Token::RParen);
                Ok(result)
            }
            _ => Err(anyhow!("Unexpected token: {:?}", self.current_token)),
        }
    }

    fn eat(&mut self, token: Token) -> Result<()> {
        if self.current_token == token {
            self.current_token = self.lexer.next_token()?;
            Ok(())
        } else {
            Err(anyhow!("Unexpected token: {:?}", self.current_token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::query::SearchQuery;

    use super::{Lexer, Parser};

    #[test]
    fn test_basic() {
        let input = String::from("word");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        assert_eq!(SearchQuery::Word("word".into()), output);
    }

    #[test]
    fn test_paren() {
        let input = String::from("(word)");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        assert_eq!(SearchQuery::Word("word".into()), output);
    }

    #[test]
    fn test_and() {
        let input = String::from("word & word2");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        let expected = SearchQuery::And(Box::new((
            SearchQuery::Word("word".to_string()),
            SearchQuery::Word("word2".to_string()),
        )));
        assert_eq!(expected, output);
    }

    #[test]
    fn test_or() {
        let input = String::from("word | word2");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        let expected = SearchQuery::Or(Box::new((
            SearchQuery::Word("word".to_string()),
            SearchQuery::Word("word2".to_string()),
        )));
        assert_eq!(expected, output);
    }

    #[test]
    fn test_or_with_paren() {
        let input = String::from("(word | (word2))");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        let expected = SearchQuery::Or(Box::new((
            SearchQuery::Word("word".to_string()),
            SearchQuery::Word("word2".to_string()),
        )));
        assert_eq!(expected, output);
    }

    #[test]
    fn test_complex() {
        let input = String::from("(word1 | word2) & (word3 | word4 | word5)");
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer).unwrap();
        let output = parser.parse().unwrap();
        let expected = SearchQuery::And(Box::new((
            SearchQuery::Or(Box::new((
                SearchQuery::Word("word1".to_string()),
                SearchQuery::Word("word2".to_string()),
            ))),
            SearchQuery::Or(Box::new((
                SearchQuery::Or(Box::new((
                    SearchQuery::Word("word3".to_string()),
                    SearchQuery::Word("word4".to_string()),
                ))),
                SearchQuery::Word("word5".to_string()),
            ))),
        )));
        assert_eq!(expected, output);
    }
}
