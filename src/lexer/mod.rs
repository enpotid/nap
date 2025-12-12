use crate::ast::*;
use std::{iter::Peekable, str::Chars};

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();

    'n: while let Some(c) = chars.next() {
        match c {
            'A'..='Z' | '_' => {
                let mut buffer = String::from(c);

                while let Some(&c2) = chars.peek() {
                    match c2 {
                        'A'..='Z' | '_' => {
                            buffer.push(c2);
                            chars.next();
                        }
                        '0'..='9' | '.' | ':' => {
                            let d = chars.next().unwrap();

                            tokens.push(Token {
                                kind: TokenKind::Ref {
                                    kind: buffer.clone(),
                                    index: tokenize_number(&mut chars, d),
                                },
                            });

                            buffer.clear();
                            continue 'n;
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if buffer == "END" {
                    tokens.push(Token {
                        kind: TokenKind::FnEnd,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Ident(buffer.clone()),
                    });
                }
                buffer.clear();
            }
            '0'..='9' | '.' | ':' => {
                tokens.push(Token {
                    kind: TokenKind::Ref {
                        kind: String::from("L"),
                        index: tokenize_number(&mut chars, c),
                    },
                });
            }
            _ => {}
        }
    }

    tokens
}

fn tokenize_number(chars: &mut Peekable<Chars<'_>>, d: char) -> String {
    let mut buffer = String::from(d);
    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' | '.' | ':' => {
                buffer.push(c);
                chars.next();
            }
            _ => {
                break;
            }
        }
    }

    buffer
}
