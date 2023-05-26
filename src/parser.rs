use crate::{Lexer, Tree, TreeKind};

pub struct Parser {
    tokens: Lexer,
}

impl Parser {
    pub fn new(tokens: Lexer) -> Self {
        Self { tokens }
    }

    pub fn build_tree(self) -> Tree {
        Tree {
            kind: TreeKind::ErrorTree,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
