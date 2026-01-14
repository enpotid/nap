use crate::ast::*;
use std::{iter::Peekable, slice::Iter};

pub fn parse(tokens: &Vec<Token>) -> Computer {
    let mut tokens_iter = tokens.iter().peekable();

    Computer {
        items: parse_block(&mut tokens_iter, true),
    }
}

pub fn parse_block(tokens_iter: &mut Peekable<Iter<'_, Token>>, eof: bool) -> Vec<Box<Item>> {
    let mut items = Vec::new();

    while let Some(t) = tokens_iter.next() {
        match t.kind {
            TokenKind::Ident(_) => {
                if let Some(t2) = tokens_iter.peek()
                    && let TokenKind::OpenBrace = t2.kind
                {
                    tokens_iter.next();
                    items.push(Box::new(Item {
                        kind: ItemKind::Fn {
                            ident: t.clone(),
                            block: parse_block(tokens_iter, false),
                        },
                    }));
                } else {
                    items.push(Box::new(Item {
                        kind: ItemKind::Call { ident: t.clone() },
                    }));
                }
            }
            TokenKind::Ref { .. } => {
                items.push(Box::new(Item {
                    kind: ItemKind::Arg { ident: t.clone() },
                }));
            }
            TokenKind::CloseBrace if !eof => return items,
            _ => todo!(), // unexpected token
        }
    }

    if eof {
        return items;
    } else {
        todo!(); // unexpected EOF while parsing function block (missing '}')
    }
}
