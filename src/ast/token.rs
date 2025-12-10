pub struct Token {
    pub kind: TokenKind,
}

pub enum TokenKind {
    Ident(String),                    // XOR, AND,,,
    Ref { kind: String, index: u64 }, // "F0", "I0", "0"...
    FnEnd,                            // END
}
