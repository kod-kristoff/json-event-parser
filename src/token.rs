#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum TokenKind {
    ErrorToken, Eof,

    LCurly, RCurly,

    JString,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
