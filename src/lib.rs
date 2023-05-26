use std::fmt;

#[derive(Debug)]
#[rustfmt::skip]
enum TreeKind {
    ErrorTree,
}

pub struct Tree {
    kind: TreeKind,
}

pub fn parse_tree(text: &str) -> Tree {
    Tree {
        kind: TreeKind::ErrorTree,
    }
}
use std::fmt::Write;
impl Tree {
    fn print(&self, buf: &mut String, level: usize) -> Result<(), fmt::Error> {
        let indent = " ".repeat(level);
        write!(buf, "{indent}{:?}\n", self.kind)?;
        Ok(())
    }
}
impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0)?;
        write!(f, "{}", buf)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
