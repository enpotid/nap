use crate::ast::Token;

pub struct Computer {
    pub items: Vec<Box<Item>>,
}

pub struct Item {
    pub kind: ItemKind,
}

pub enum ItemKind {
    Fn { ident: Token, block: Vec<Box<Item>> },
    Call { ident: Token, args: Vec<Token> },
}
