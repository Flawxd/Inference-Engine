use crate::types::*;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Atom(String),
    Variable(String),
    LParen,
    RParen,
    Comma,
    Dot,
    ColonDash, // :-
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char),
    UnterminatedQuotedAtom,
    UnexpectedTokenAfterColon,
    UnexpectedToken { expected: String, got: String },
    UnexpectedEof,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedChar(c) => write!(f, "Unexpected character: {:?}", c),
            ParseError::UnterminatedQuotedAtom => write!(f, "Unterminated quoted atom"),
            ParseError::UnexpectedTokenAfterColon => write!(f, "Unexpected character after ':'"),
            ParseError::UnexpectedToken { expected, got } => {
                write!(f, "Expected {}, got {}", expected, got)
            }
            ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
        }
    }
}


fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }

            '%' => while chars.next().map(|c| c != '\n').unwrap_or(false) {},

            '(' => { tokens.push(Token::LParen);  chars.next(); }
            ')' => { tokens.push(Token::RParen);  chars.next(); }
            ',' => { tokens.push(Token::Comma);   chars.next(); }
            '.' => { tokens.push(Token::Dot);     chars.next(); }

            ':' => {
                chars.next();
                if chars.peek() == Some(&'-') {
                    chars.next();
                    tokens.push(Token::ColonDash);
                } else {
                    return Err(ParseError::UnexpectedTokenAfterColon);
                }
            }

            '\'' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('\'') => break,
                        Some(c)    => s.push(c),
                        None       => return Err(ParseError::UnterminatedQuotedAtom),
                    }
                }
                tokens.push(Token::Atom(s));
            }

            c if c.is_ascii_lowercase() => {
                let mut s = String::new();
                s.push(chars.next().unwrap());
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '_').unwrap_or(false) {
                    s.push(chars.next().unwrap());
                }
                tokens.push(Token::Atom(s));
            }

            c if c.is_ascii_uppercase() || c == '_' => {
                let mut s = String::new();
                s.push(chars.next().unwrap());
                while chars.peek().map(|c| c.is_alphanumeric() || *c == '_').unwrap_or(false) {
                    s.push(chars.next().unwrap());
                }
                tokens.push(Token::Variable(s));
            }

            other => return Err(ParseError::UnexpectedChar(other)),
        }
    }

    Ok(tokens)
}


struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        if self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Ok(tok)
        } else {
            Err(ParseError::UnexpectedEof)
        }
    }

    fn expect(&mut self, expected: &Token) -> Result<(), ParseError> {
        let tok = self.next_token()?;
        if &tok == expected {
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                got:      format!("{:?}", tok),
            })
        }
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        match self.next_token()? {
            Token::Variable(name) => Ok(Term::Variable(name)),
            Token::Atom(name) => {
                if self.peek() == Some(&Token::LParen) {
                    self.expect(&Token::LParen)?;
                    let mut args = vec![self.parse_term()?];
                    while self.peek() == Some(&Token::Comma) {
                        self.expect(&Token::Comma)?;
                        args.push(self.parse_term()?);
                    }
                    self.expect(&Token::RParen)?;
                    Ok(Term::Compound { functor: name, args })
                } else {
                    Ok(Term::Atom(name))
                }
            }
            other => Err(ParseError::UnexpectedToken {
                expected: "a term".into(),
                got:      format!("{:?}", other),
            }),
        }
    }

    fn parse_term_list(&mut self) -> Result<Vec<Term>, ParseError> {
        let mut terms = vec![self.parse_term()?];
        while self.peek() == Some(&Token::Comma) {
            self.expect(&Token::Comma)?;
            terms.push(self.parse_term()?);
        }
        Ok(terms)
    }

    fn parse_clause(&mut self) -> Result<(Option<Fact>, Option<Rule>), ParseError> {
        let head = self.parse_term()?;
        match self.peek() {
            Some(Token::Dot) => {
                self.expect(&Token::Dot)?;
                Ok((Some(Fact { term: head }), None))
            }
            Some(Token::ColonDash) => {
                self.expect(&Token::ColonDash)?;
                let body = self.parse_term_list()?;
                self.expect(&Token::Dot)?;
                Ok((None, Some(Rule { head, body })))
            }
            other => Err(ParseError::UnexpectedToken {
                expected: "'.' or ':-'".into(),
                got:      format!("{:?}", other),
            }),
        }
    }

    fn parse_knowledge_base(&mut self) -> (KnowledgeBase, Option<String>) {
        let mut kb = KnowledgeBase::new();
        while self.peek().is_some() {
            match self.parse_clause() {
                Ok((Some(fact), None)) => {kb.add_fact(fact);},
                Ok((None, Some(rule))) => kb.add_rule(rule),
                Ok(_) => break,
                Err(e) => return (kb, Some(e.to_string())),
            }
        }
        (kb, None)
    }
}

pub fn parse(input: &str) -> (KnowledgeBase, Option<String>) {
    match tokenize(input) {
        Err(e) => (KnowledgeBase::new(), Some(e.to_string())),
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            parser.parse_knowledge_base()
        }
    }
}