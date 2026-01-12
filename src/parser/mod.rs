use crate::ast::*;
use std::{iter::Peekable, slice::Iter};

pub fn parse(tokens: &Vec<Token>) -> Computer {
    let mut items = Vec::new();
    let mut tokens_iter = tokens.iter().peekable();

    while let Some(t) = tokens_iter.next() {
        if let TokenKind::Ident(ident) = t.kind.clone() {
            if let Some(t2) = tokens_iter.peek() {
                if let TokenKind::Ref { .. } = t2.kind.clone() {
                    items.push(Box::new(parse_call(&tokens_iter, ident)));
                } else if let TokenKind::FnSymbol { .. } = t2.kind.clone() {
                    items.push(Box::new(parse_fn(&tokens_iter, ident)));
                } else {
                    // TODO
                }
            } else {
                // TODO
            }
        } else if let TokenKind::Ref { .. } = t.kind.clone() {
            if let Some(&t2) = tokens_iter.peek() {
                if let TokenKind::Ref { .. } = t2.kind.clone() {
                    items.push(Box::new(Item {
                        kind: ItemKind::Call {
                            ident: None,
                            args: vec![t.clone(), t2.clone()],
                        },
                    }));
                } else {
                    // TODO
                }
            } else {
                // TODO
            }
        } else {
            // TODO
        }
    }

    Computer { items }
}

pub fn parse_call(tokens_iter: &Peekable<Iter<'_, Token>>, ident: String) -> Item {}

pub fn parse_fn(tokens_iter: &Peekable<Iter<'_, Token>>, ident: String) -> Item {}
