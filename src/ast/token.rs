#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
    Ident(String),                       // XOR, AND,,,
    Ref { kind: String, index: String }, // "F0", "I0", "0", "0.1"...
    FnEnd,                               // END
}
