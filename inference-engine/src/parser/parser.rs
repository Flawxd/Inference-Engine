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

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Skip comments
            '%' => while chars.next().map(|c| c != '\n').unwrap_or(false) {},

            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '.' => {
                tokens.push(Token::Dot);
                chars.next();
            }

            ':' => {
                chars.next();
                if chars.peek() == Some(&'-') {
                    chars.next();
                    tokens.push(Token::ColonDash);
                } else {
                    panic!("Unexpected character after ':'");
                }
            }

            // Quoted atoms: 'like this'
            '\'' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('\'') => break,
                        Some(c) => s.push(c),
                        None => panic!("Unterminated quoted atom"),
                    }
                }
                tokens.push(Token::Atom(s));
            }

            // Atoms start with a lowercase letter
            c if c.is_ascii_lowercase() => {
                let mut s = String::new();
                while chars
                    .peek()
                    .map(|c| c.is_alphanumeric() || *c == '_')
                    .unwrap_or(false)
                {
                    s.push(chars.next().unwrap());
                }
                tokens.push(Token::Atom(s));
            }

            // Variables start with uppercase or _
            c if c.is_ascii_uppercase() || c == '_' => {
                let mut s = String::new();
                while chars
                    .peek()
                    .map(|c| c.is_alphanumeric() || *c == '_')
                    .unwrap_or(false)
                {
                    s.push(chars.next().unwrap());
                }
                tokens.push(Token::Variable(s));
            }

            other => panic!("Unexpected character: {:?}", other),
        }
    }

    tokens
}

// ─── Parser ──────────────────────────────────────────────────────────────────

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

    fn next(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) {
        let tok = self.next();
        assert_eq!(&tok, expected, "Expected {:?}, got {:?}", expected, tok);
    }

    /// Parse a single Term: atom, variable, or compound functor(args…)
    fn parse_term(&mut self) -> Term {
        match self.next() {
            Token::Variable(name) => Term::Variable(name),
            Token::Atom(name) => {
                // Compound if followed by '('
                if self.peek() == Some(&Token::LParen) {
                    self.expect(&Token::LParen);
                    let mut args = vec![self.parse_term()];
                    while self.peek() == Some(&Token::Comma) {
                        self.expect(&Token::Comma);
                        args.push(self.parse_term());
                    }
                    self.expect(&Token::RParen);
                    Term::Compound {
                        functor: name,
                        args,
                    }
                } else {
                    Term::Atom(name)
                }
            }
            other => panic!("Expected a term, got {:?}", other),
        }
    }

    /// Parse a comma-separated list of terms (rule body)
    fn parse_term_list(&mut self) -> Vec<Term> {
        let mut terms = vec![self.parse_term()];
        while self.peek() == Some(&Token::Comma) {
            self.expect(&Token::Comma);
            terms.push(self.parse_term());
        }
        terms
    }

    /// Parse one clause: fact `head.` or rule `head :- body.`
    fn parse_clause(&mut self) -> Result<(Option<Fact>, Option<Rule>), ()> {
        if self.peek().is_none() {
            return Err(());
        }
        let head = self.parse_term();
        match self.peek() {
            Some(Token::Dot) => {
                self.expect(&Token::Dot);
                Ok((Some(Fact { term: head }), None))
            }
            Some(Token::ColonDash) => {
                self.expect(&Token::ColonDash);
                let body = self.parse_term_list();
                self.expect(&Token::Dot);
                Ok((None, Some(Rule { head, body })))
            }
            other => panic!("Expected '.' or ':-', got {:?}", other),
        }
    }

    /// Parse the entire input into a KnowledgeBase
    fn parse_knowledge_base(&mut self) -> KnowledgeBase {
        let mut kb = KnowledgeBase::new();
        while self.peek().is_some() {
            match self.parse_clause() {
                Ok((Some(fact), None)) => {kb.add_fact(fact);}
                Ok((None, Some(rule))) => kb.add_rule(rule),
                _ => break,
            }
        }
        kb
    }
}

pub fn parse(input: &str) -> KnowledgeBase {
    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);
    parser.parse_knowledge_base()
}
