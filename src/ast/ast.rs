use crate::ast::Token;

#[derive(Debug)]
pub struct Computer {
    pub items: Vec<Box<Item>>,
}

#[derive(Debug)]
pub struct Item {
    pub kind: ItemKind,
}

#[derive(Debug)]
pub enum ItemKind {
    Fn { ident: Token, block: Vec<Box<Item>> },
    Call { ident: Token },
    Arg { ident: Token },
}
