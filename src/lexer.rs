use crate::Token;
use crate::TokenKind::*;

pub struct Lexer<'text> {
    text: &'text str,
}

impl<'text> Lexer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self { text }
    }

    pub fn lex(self) -> Vec<Token> {
        let punctuation = ("{ }", [LCurly, RCurly]);
        let mut text = self.text;

        let mut result = Vec::new();
        while !text.is_empty() {
            let text_orig = text;
            let mut kind = 'kind: {
                for (i, symbol) in punctuation.0.split_ascii_whitespace().enumerate() {
                    if let Some(rest) = text.strip_prefix(symbol) {
                        text = rest;
                        break 'kind punctuation.1[i];
                    }
                }
                if let Some(rest) = trim(text) {
                    text = rest;
                    break 'kind JString;
                }
                let error_index = text
                    .find(|it: char| it.is_ascii_whitespace())
                    .unwrap_or(text.len());
                text = &text[error_index..];
                ErrorToken
            };
            assert!(text.len() < text_orig.len());
            let token_text = &text_orig[..text_orig.len() - text.len()];
            result.push(Token {
                kind,
                text: token_text.to_string(),
            });
        }

        dbg!(&result);
        return result;
    }
}

pub fn trim(text: &str, predicate: impl std::ops::Fn(char) -> bool) -> Option<&str> {
    let index = text
        .find(|it: char| !predicate(it))
        .unwrap_or(text.length());
    if index == 0 {
        None
    } else {
        Some(&text[index..])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
