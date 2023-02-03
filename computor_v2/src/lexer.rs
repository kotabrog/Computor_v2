use std::str::Chars;
use std::iter::Peekable;


#[derive(Debug, PartialEq)]
pub enum Token {
    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]
    Comma, // ,
    SemiColon, // ;
    Caret, // ^
    TwoAsterisk, // **
    Asterisk, // *
    Slash, // /
    Percent, // %
    Plus, // +
    Minus, // -
    I, // i
    Equal, // =
    Question, // ?
    NumString(Box<String>), // [0-9]*.?[0-9]*
    String(Box<String>), // [A-Za-z]* -> [a-z]*
}


pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

enum PendingType {
    Asterisk,
    NumString,
    String,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer { iter: input.chars().peekable() }
    }

    pub fn make_token_vec(&mut self) -> Result<Vec<Token>, String> {
        let mut vec = Vec::new();
        loop {
            match self.next_token()? {
                None => return Ok(vec),
                Some(t) => vec.push(t),
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, String> {
        let mut pending_string = String::new();
        let mut dot_flag = false;
        let pending_flag = loop {
            match self.next() {
                None => return Ok(None),
                Some(c) => {
                    if c.is_ascii_whitespace() {
                        continue;
                    }
                    match c {
                        '(' => return Ok(Some(Token::LParen)),
                        ')' => return Ok(Some(Token::RParen)),
                        '[' => return Ok(Some(Token::LBracket)),
                        ']' => return Ok(Some(Token::RBracket)),
                        ',' => return Ok(Some(Token::Comma)),
                        ';' => return Ok(Some(Token::SemiColon)),
                        '^' => return Ok(Some(Token::Caret)),
                        '/' => return Ok(Some(Token::Slash)),
                        '%' => return Ok(Some(Token::Percent)),
                        '+' => return Ok(Some(Token::Plus)),
                        '-' => return Ok(Some(Token::Minus)),
                        '=' => return Ok(Some(Token::Equal)),
                        '?' => return Ok(Some(Token::Question)),
                        'i' => return Ok(Some(Token::I)),
                        'I' => return Err(format!("Unsupported character: {}", c)),
                        '*' => {
                            pending_string.push(c);
                            break PendingType::Asterisk;
                        },
                        '0'..='9' => {
                            pending_string.push(c);
                            break PendingType::NumString;
                        }
                        'a'..='z' | 'A'..='Z' => {
                            pending_string.push(c);
                            break PendingType::String;
                        }
                        _ => return Err(format!("Unsupported character: {}", c)),
                    }
                },
            }
        };
        loop {
            match self.peek() {
                None => {
                    match pending_flag {
                        PendingType::Asterisk => return Ok(Some(Token::Asterisk)),
                        PendingType::NumString => return Ok(Some(Token::NumString(Box::new(pending_string)))),
                        PendingType::String => return Ok(Some(Token::String(Box::new(pending_string)))),
                    }
                }
                Some(c) => {
                    if c.is_ascii_whitespace() {
                        match pending_flag {
                            PendingType::Asterisk => return Ok(Some(Token::Asterisk)),
                            PendingType::NumString => return Ok(Some(Token::NumString(Box::new(pending_string)))),
                            PendingType::String => return Ok(Some(Token::String(Box::new(pending_string)))),
                        }
                    }
                    match c {
                        '(' | ')' | '[' | ']' | ',' | ';' | '^' | '/' | '%' | '+' | '-' | '=' | '?' | 'i' | 'I' => {
                            match pending_flag {
                                PendingType::Asterisk => return Ok(Some(Token::Asterisk)),
                                PendingType::NumString => return Ok(Some(Token::NumString(Box::new(pending_string)))),
                                PendingType::String => return Ok(Some(Token::String(Box::new(pending_string)))),
                            }
                        },
                        '*' => {
                            match pending_flag {
                                PendingType::Asterisk => {
                                    self.next();
                                    return Ok(Some(Token::TwoAsterisk))
                                },
                                PendingType::NumString => return Ok(Some(Token::NumString(Box::new(pending_string)))),
                                PendingType::String => return Ok(Some(Token::String(Box::new(pending_string)))),
                            }
                        },
                        '.' => {
                            match pending_flag {
                                PendingType::NumString => {
                                    if dot_flag {
                                        return Err("'.' is in the wrong position.".to_string())
                                    }
                                    self.next();
                                    dot_flag = true;
                                    pending_string.push('.');
                                },
                                _ => return Err("'.' is in the wrong position.".to_string()),
                            }
                        }
                        '0'..='9' => {
                            match pending_flag {
                                PendingType::Asterisk => return Ok(Some(Token::Asterisk)),
                                PendingType::NumString => {
                                    let c = self.next().unwrap();
                                    pending_string.push(c);
                                },
                                PendingType::String => return Ok(Some(Token::String(Box::new(pending_string)))),
                            }
                        }
                        'a'..='z' | 'A'..='Z' => {
                            match pending_flag {
                                PendingType::Asterisk => return Ok(Some(Token::Asterisk)),
                                PendingType::NumString => return Ok(Some(Token::NumString(Box::new(pending_string)))),
                                PendingType::String => {
                                    let c = self.next().unwrap();
                                    pending_string.push(c);
                                },
                            }
                        }
                        _ => return Err(format!("Unsupported character: {}", c)),
                    }
                }
            }
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.iter.next()
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_all() {
        use Token::*;
        let s = "()^*/%**+-i=?[],;a1A2zz ZZ123.098^A".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![LParen, RParen, Caret, Asterisk, Slash, Percent,
                                TwoAsterisk, Plus, Minus, I, Equal, Question,
                                LBracket, RBracket, Comma, SemiColon,
                                String(Box::new("a".to_string())),
                                NumString(Box::new("1".to_string())),
                                String(Box::new("A".to_string())),
                                NumString(Box::new("2".to_string())),
                                String(Box::new("zz".to_string())),
                                String(Box::new("ZZ".to_string())),
                                NumString(Box::new("123.098".to_string())),
                                Caret,
                                String(Box::new("A".to_string()))
        ]));
    }

    #[test]
    fn lexer_empty() {
        let s = "".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![]));
    }

    #[test]
    fn lexer_empty_empty() {
        let s = "".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![]));
    }

    #[test]
    fn lexer_solo_solo() {
        use Token::*;
        let s = "^".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![Caret]));
    }

    #[test]
    fn lexer_solo_asterisk() {
        use Token::*;
        let s = "*".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![Asterisk]));
    }

    #[test]
    fn lexer_solo_two_asterisk() {
        use Token::*;
        let s = "**".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![TwoAsterisk]));
    }

    #[test]
    fn lexer_solo_alphabet() {
        use Token::*;
        let s = "x".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![String(Box::new("x".to_string()))]));
    }

    #[test]
    fn lexer_solo_num() {
        use Token::*;
        let s = "1".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Ok(vec![NumString(Box::new("1".to_string()))]));
    }

    #[test]
    fn lexer_unsupported_character() {
        let s = "aa}aa".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Err("Unsupported character: }".to_string()));
    }

    #[test]
    fn lexer_unsupported_character_i() {
        let s = "aaIaa".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Err("Unsupported character: I".to_string()));
    }

    #[test]
    fn lexer_unsupported_dot() {
        let s = "aa.aa".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Err("'.' is in the wrong position.".to_string()));
    }

    #[test]
    fn lexer_unsupported_two_dot() {
        let s = "aa1..2aa".to_string();
        let mut lexer = Lexer::new(&s);
        let vec = lexer.make_token_vec();
        assert_eq!(vec, Err("'.' is in the wrong position.".to_string()));
    }
}