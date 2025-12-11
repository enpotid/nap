use crate::ast::*;
use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    pub tokens: Vec<Token>,
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Cursor {
            tokens: Vec::new(),
            chars: src.chars().peekable(),
        }
    }

    pub fn read(&mut self) {
        while let Some(c) = self.chars.next() {
            match c {
                _ => {}
            }
        }
    }
}
