mod tree;
pub use tree::{Tree, TreeKind};

mod lexer;
use lexer::Lexer;

mod parser;
use parser::Parser;

mod rules;

pub fn parse_tree(text: &str) -> Tree {
    let tokens = Lexer::new(text);
    let mut p = Parser::new(tokens);
    rules::start(&mut p);
    p.build_tree()
}

#[macro_export]
macro_rules! format_to {
    ($buf:expr) => ();
    ($buf:expr, $lit:literal, $($arg:tt)*) => {
        { use ::std::fmt::Write as _; ::std::write!($buf, $lit, $($arg)*)?; }

    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
